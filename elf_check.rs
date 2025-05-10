use goblin::Object ;
use std::fs ;
fn main(){
    let some_binary = fs::read("main").expect("Failed !"); //use a compiled file insted of main file
    match Object::parse(&some_binary){
        Ok(obj) => match obj {
            Object::Elf(_) => println!("This is a elf file !"),
            _ => println!("This is not a elf file"),
        },
        Err(e) => {
            eprintln!("Error :{}" , e);
        }
    }
}
