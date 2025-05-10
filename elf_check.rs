use goblin::Object;
use std::fs;

fn main() {
    let bytes = fs::read("main").expect("Cannot read file"); // use compiled file insted of main file 
    match Object::parse(&bytes).unwrap() {
        Object::Elf(_) => println!("This is an ELF file."),
        _ => println!("Not an ELF file."),
    }
}
