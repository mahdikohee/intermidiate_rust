--In this file i will do all rust code form the first but in extreme explicit way 
//A simple vector and for loop 
use std::vec::Vec ;
use std::ops::RangeInclusive ;
fn main(){
    let mut vector : Vec<i32> = Vec::<i32>::new() ;
    let num_range : RangeInclusive<i32> = RangeInclusive::<i32>::new(1i32 ,10i32) ;
    for i in num_range{
        let double : i32 = i * 2 ;
        Vec::<i32>::push(&mut vector , double) ;
    }
    println!("Your vector is {:?}" , vector) ;
}
