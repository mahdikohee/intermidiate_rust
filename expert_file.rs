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



//another example 
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}


//another example 
use std::fs::File ;
use std::io::Read ;
fn main() -> std::io::Result<()>{
    let mut file = File::open("output.txt").expect("Failed @Q");
    let mut content = String::new();
    match file.read_to_string(&mut content){
        Ok(_) => println!("Your data is :{}" , content),
        Err(e) => {
            eprintln!("Error as e :{:?}" , e);
        }
    }
    Ok(())
}




///another example
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("output.bin").expect("Failed to open file!");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read file!");  //use read_to_end insted of read_to_string to read the raw binary file content 

    println!("Read {} bytes from output.bin", data.len());
    Ok(())
}

////another example 
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut f = File::create("foo.txt")?;
    f.write_all(&1234_u32.to_be_bytes())?;    //see explanation why using to_be_bytes()
    Ok(())
}
............................................explanation.................................
to_be_bytes() means big endian ...means the bigh byte will load first
to_le_bytes() means little endian ...means the little byte will load first 
why using endian insted of as_bytes() ?
answer => when its comes to manupulate fixed size then use endian 
and otherwise use as_bytes() for most string manupulation .
    
কেন আলাদা?
এটা CPU architecture-এর উপর নির্ভর করে।

Intel = Little Endian

Network protocols, কিছু embedded systems = Big Endian
"Network Byte Order" মানেই Big Endian

///another example 
