-------------this is for to understand all and everything of explicit typing of rust-----------------------
target => fuck cpp , -000000.1% bug , verbose code to understand what is hapenning , and mainly make rust without any hidden controll like zig and c 
//simple example of iter
fn main(){
    let mut vector : Vec<i32> = Vec::new() ;
    for i in 1..=10{
        vector.push(i * 2) ;
    }
    if !vector.is_empty(){
        println!(" ") ;
    }else{
        eprintln!("Your vector is empty !") ;
    }
    let mut  vec_iter : std::vec::IntoIter<i32> = vector.into_iter() ;
    let main_val : Option<i32> = vec_iter.nth(1) ;
    match main_val{
        Some(val) => println!("Your value is {:?}" , val) ,
        None => eprintln!("No value found yet !") ,
    }
}
//another simple example 
fn main(){
    let vector : Vec<i32> = vec![1 , 2 , 3] ;
    assert_eq!(None , vector.into_iter().nth(4)) ;  ----Because .nth() returns Option<> type and there is no value at index 4....so its None
}

//explicit way for a s simple for-loop 
fn main(){
    let number : std::ops::Range<i32> = 1..10 ;
    for i in number{
        println!("{:?}" , i) ;
    }
}
