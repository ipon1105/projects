use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err: &str| {
        eprintln!("[Ошибка|Входных параметров] {err}");
        process::exit(1);
    });

    println!("Ищем это:\t{}", &config.query);
    println!("Ищем здесь:\t{}",&config.file_path);

    if let Err(e) = run(config){
        eprintln!("[Ошибка|Обработки] {e}");
        process::exit(1);
    }
}

