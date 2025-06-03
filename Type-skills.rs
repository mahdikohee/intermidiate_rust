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
