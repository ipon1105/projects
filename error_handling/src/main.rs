use std::{fs::File, io::{ErrorKind, Read}};
use std::error::Error;

struct  {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Self {
        if value > 0 && value < 101 {
            return Self{value}
        } 
        return Self { value: 3 }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Когда стоит использовать unwrap и expect
    // Пользовательский тип для проверки

    let g = 100;

    let mut name: String = String::new();
    let file = File::open("5.txt").unwrap();
    // File::open("file.txt")?.read_to_string(&mut name)?;

    println!("name = {}", name);
    Ok(())
}
