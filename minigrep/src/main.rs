use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let temple = args.get(1).expect("Неверные параметры программы");
    let filename = args.get(2).expect("Неверные параметры программы");

    dbg!(temple);
    dbg!(filename);

    let content = fs::read_to_string(filename).expect("Невозможно открыть файл");
    println!("\n\nСодержимое файла: {}\n", content);
}

