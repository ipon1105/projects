use std::{fs::File, io::{ErrorKind, Read}};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut name: String = String::new();
    File::open("file.txt")?.read_to_string(&mut name)?;

    println!("name = {}", name);
    Ok(())
}
