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
    let result = read_num_from_file("num.txt");
    println!("Your number is :{:?}" , result);
    Ok(())
}


ex---4
fn extract_prefix(s: &str, len: usize) -> Option<String> {
    let mut chars = s.chars();
    let prefix: String = chars.by_ref().take(len).collect();
    
    // নিশ্চিত করি যে আমরা যথেষ্ট character পেয়েছি
    if prefix.chars().count() == len {
        Some(prefix)
    } else {
        None
    }
}

fn main() {
    let input = String::from("নমস্কারRustaceans"); // বাংলা + ইংরেজি
    match extract_prefix(&input, 5) {
        Some(prefix) => println!("Prefix: {}", prefix),
        None => println!("String too short!"),
    }
}
...



another excercise about encrypt and decrypt a text string using windows dpapi 


use windows_dpapi::{encrypt_data , decrypt_data , Scope};
fn dec_enc_data(target_data : &[u8]) -> Result<String , Box<dyn std::error::Error>>{
    let do_encrypt = encrypt_data(target_data , Scope::User).expect("Failed to encrypt data !");
    let do_decrypt = decrypt_data(&do_encrypt , Scope::User).expect("Failed to decrypt data !");
    let result = String::from_utf8_lossy(&do_decrypt);
    Ok(result.to_string())
}
fn main() -> anyhow::Result<()> {
    let data_byte = b"Hello kohee and dibbo";
    let main_result = dec_enc_data(data_byte);
    match main_result{
        Ok(data) => {

            println!("Your decrypted dats is :{:?}" , data);
        }
        Err(e) => {
            eprintln!("Error as e :{:?}" , e);
        }
    }
    Ok(())
}




