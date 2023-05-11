use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

// Ассоциативный метод
impl Config { 
    pub fn build(arr: &[String]) -> Result<Config, &'static str> {
        if arr.len() < 3 {
            return Err("Использование: minigrep {{Запрос}} {{Файл}}");
        }
        // let temple = args.get(1).expect("Неверные параметры программы");
        // let filename = args.get(2).expect("Неверные параметры программы");
    
        let query: String = arr[1].clone();
        let file_path: String = arr[2].clone();
    
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content:String = fs::read_to_string(config.file_path)?;

    for i in search(&config.query, &content) {
        println!("{i}");
    }
    // println!("\n\nСодержимое файла:\n{}", content);
    
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res:Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let content: &str = "\
Rust:
safe, fast, productive.
Pick three.
"; 

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}