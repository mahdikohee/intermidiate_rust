///ex for learning error handeling and returning value 
use anyhow::{Result, anyhow};

fn might_fail(i: i32) -> Result<String> {
    if i == 42 {
        Ok("Everything is fine!".to_string())
    } else {
        Err(anyhow!("Something went wrong. Input was: {}", i))
    }
}

fn main() -> Result<()> {        //here use only Result<()> not std::io::Result;
                                  //because we have already implimented anyhow::Result<String>  error type 
    let val = might_fail(10)?;
    println!("Success: {}", val);
    Ok(())
}

//ex -2 
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = sum(10, 20);
    println!("Sum is: {}", result);
}

//ex-3 
fn find_even_number(numbers: &[i32]) -> Option<i32> {
    for num in numbers {
        if num % 2 == 0 {
            return Some(*num);
        }
    }
    None
}

fn main() {
    let nums = vec![1, 3, 7, 10];
    if let Some(even) = find_even_number(&nums) {
        println!("First even number: {}", even);
    } else {
        println!("No even number found");
    }
}



ex-3 
use std::fs::File ;
use std::io::{ BufRead , BufReader};
use std::error::Error ;
fn read_num_from_file(path : &str) -> Result<Vec<i32> , Box<dyn Error>> {

    let file = File::open(&path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut number = Vec::new();
    for line in reader.lines(){

        let line = line?;
        if let Ok(num) = line.trim().parse::<i32>(){

            number.push(num);
        }else {

            eprintln!("Warning skeping invalid number not i32 :{}" , line);
        }
    }
    Ok(number)
}
fn main() -> Result<() , Box<dyn Error>>{


//ex-4 
    
    let result = read_num_from_file("num.txt");
    println!("Your number is :{:?}" , result);
    Ok(())
}
