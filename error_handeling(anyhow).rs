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
