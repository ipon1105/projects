use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// Ассоциативный метод
impl Config { 
    pub fn build(mut arr: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        arr.next();
        
        let query = match arr.next() {
            Some(arg) => arg,
            None => return Err("Использование: minigrep {{Запрос}} {{Файл}}"),
        };

        let file_path = match arr.next() {
            Some(arg) => arg,
            None => return Err("Использование: minigrep {{Запрос}} {{Файл}}"),
        };
        
        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content:String = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query,  &content)
    } else {
        search_case_sensitive(&config.query, &content)
    };

    for i in result {
        println!("{i}");
    }
    // println!("\n\nСодержимое файла:\n{}", content);
    
    Ok(())
}

fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res:Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }

    res
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res:Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let content: &str = "\
Rust:
safe, fast, productive.
Duck three.
"; 

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, content));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUst";
        let content: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
"; 

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}