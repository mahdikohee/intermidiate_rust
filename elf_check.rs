use goblin::Object;
use std::fs;

fn main() {
    let bytes = fs::read("some_binary").expect("Cannot read file");

    match Object::parse(&bytes) {
        Ok(obj) => match obj {
            Object::Elf(_) => println!("This is an ELF file."),
            _ => println!("Not an ELF file."),
        },
        Err(e) => println!("Error parsing object: {}", e),
    }
}
