//rust static typing  skills is extremely important for reverse engenering like code breaking and sym exe 
//u32 of rust 
fn main() {
    let n = 0b01001100u32;
    println!("{}", n.count_ones()); // 3

    let max = u32::MAX;
    println!("{}", max.count_ones()); // 32

    let zero = 0u32;
    println!("{}", zero.count_ones()); // 0
}

//u32 for to count zeros 
fn main() {
    let zero = 0u32;
    println!("{}", zero.count_zeros()); // 32

    let max = u32::MAX;
    println!("{}", max.count_zeros()); // 0
}


//a simple rust code to convert a binary to an integer 
fn main() {
    let binary_input = "01001100";

    match u32::from_str_radix(binary_input, 2) {
        Ok(decimal) => println!("Binary: {} -> Decimal: {}", binary_input, decimal),
        Err(_) => println!("Invalid binary input, DD..."),
    }
}

//another example
use std::mem::size_of;

fn main() {
    println!("Size of u8: {} bytes", size_of::<u8>());
    println!("Size of i32: {} bytes", size_of::<i32>());
    println!("Size of bool: {} bytes", size_of::<bool>());
}

