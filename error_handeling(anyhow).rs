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

