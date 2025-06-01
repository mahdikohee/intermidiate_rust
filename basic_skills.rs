//Practise of "Some" and "None" 
fn maybe_get_number(get_it: bool) -> Option<i32> {
    if get_it {
        Some(42)
    } else {
        None
    }
}

fn main() {
    let result = maybe_get_number(true);

    match result {
        Some(n) => println!("Got a number: {}", n),
        None => println!("Got nothing"),
    }
}

//some vec 
fn main() {
    let vector = vec![1, 2, 3, 4, 5];

    // রেফারেন্স দিয়ে সরাসরি ইন্ডেক্সে এক্সেস
    let third_element: &i32 = &vector[2];
    println!("The third element is: {:?}", third_element);

    // .get() রিটার্ন করে Option<&i32>
    let third_element: Option<&i32> = vector.get(2);
    match third_element {
        Some(value) => println!("The third element is: {:?}", value),
        None => println!("There is no third element!"),
    }
}

//some vec ex
fn main() {
    let vector: Vec<i32> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 1000];
    let filtered: Vec<i32> = vector.into_iter().filter(|&x| x < 60).collect();
    
    if !filtered.is_empty() {
        println!("Less than 60 are: {:?}", filtered);
    } else {
        println!("No value found!");
    }
}

//some vec ex 
fn main(){

    let mut vector : Vec<i32> = Vec::new();
    for i in 1..100{

        vector.push(i * 3);
    }
    println!("The value has pussed !");
    println!("{:?}" , vector);
    let filter  : Vec<i32> = vector.into_iter().filter(|&x| x > 20 ).collect();
    if !filter.is_empty(){

        println!("The value of vector greater then 20 is :{:?}" , filter);
    }else {

        println!("No value found greater then 20 !");
    }
}


//more ex of vec
fn main(){

    let vector : Vec<&str> = vec!["I" , "am" , "the" , "Best"] ; 
    let sentence :&str = &vector.join(" ");   //by default join makes String not str ..if we set the type as &str then we have to use &[the veriable] like &vector
    println!("The sentence is :{:?}" , sentence);
}

//the default example of join is 
fn main(){

    let vector : Vec<&str> = vec!["I" , "am" , "the" , "best"]; 
    let sentence : String = vector.join(" ");                  //join makes String by default 
    println!("The sentence is :{:?}" , sentence);
}

//A Simple hashmap 
use std::collections::HashMap ; 
fn main(){
    let mut people : HashMap<String , i32> = HashMap::new();
    people.insert(String::from("Kohee") , 21); 
    people.insert(String::from("Sohee") , 32) ; 
    people.insert(String::from("Rahi") , 43); 

    for (name , age) in &people{

        println!("Name is :{:?} and age :{:?}" , name , age)
    }
}


note ==> String::new() creates a empty string ....String::from() creates a string which already has a string value ..its not empty 


//Option<i32> 
fn main() {

    let conf_port : Option<i32> = None ;
    let port = conf_port.unwrap_or_else(|| {
        println!("No port has seen yet !");
        9090
    });
    println!("Your conf message -> {:?}" , port);
}



//random number 
use rand::rngs::ThreadRng;
use rand::RngCore;

fn main() {
    // নতুন ThreadRng তৈরি
    let mut rng: ThreadRng = ThreadRng::default();

    // next_u32() থেকে নম্বর নিয়ে টাইপ কাস্ট করে র‍্যান্ডম তৈরি করা
    let n1: u8 = (rng.next_u32() % 256) as u8;
    let n2: u16 = (rng.next_u32() % 65536) as u16;

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.next_u32());

    // i32 তৈরি করার জন্য next_u32() থেকে কাস্ট করে নিতে হবে
    let n_i32: i32 = rng.next_u32() as i32;
    println!("Random i32: {}", n_i32);

    // f64 তৈরি করার জন্য u32 কে 0..1 স্কেলে রূপান্তর করা হয়
    let n_f64: f64 = (rng.next_u32() as f64) / (u32::MAX as f64);
    println!("Random float: {}", n_f64);
}


//get your mac address -----with iter 
use mac_address::MacAddressIterator;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    match MacAddressIterator::new() {
        Ok(addrs) => {
            for addr in addrs {
                println!("Your address is {:?}", addr.to_string());
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
    Ok(())
}

