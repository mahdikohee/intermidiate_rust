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


////another wau to get mac address 
use mac_address::get_mac_address;

fn main() {
    match get_mac_address() {
        Ok(Some(ma)) => {
            println!("MAC addr = {}", ma);
            println!("bytes = {:?}", ma.bytes());
        }
        Ok(None) => println!("No MAC address found."),
        Err(e) => println!("{:?}", e),
    }
}

//get mac address by usign another way by pnet , and datalink 
use pnet::datalink ;
use mac_address::mac_address_by_name ;
fn main(){
    for iface in datalink::interfaces(){
        if iface.is_loopback(){
            continue ;
        }
        println!("I-face is {:?}" ,iface.name) ;
        match mac_address_by_name(&iface.name){
            Ok(Some(mac)) => {
                println!{"Interface is {:?}" , iface.name} ;
                println!("And mac addr of that is {:?}" , mac.to_string()) ;
                return ;
            }
            Ok(None) => {
                println!("No mac address has found in {:?}" , iface.name)
            }
            Err(e) => {
                eprintln!("Error while getting mac address of {:?} :{:?}" , iface ,e);
            }
        }
    }
}



//same code by with Box dyn error handleing method 
use pnet::datalink;
use mac_address::mac_address_by_name;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    for iface in datalink::interfaces() {
        if iface.is_loopback() {
            continue;
        }

        println!("Your interface has found: {:?}", iface.name);

        // MAC address তোলা
        match mac_address_by_name(&iface.name) {
            Ok(Some(mac)) => {
                println!("Your interface is: {:?}", iface.name);
                println!("And your mac addr is: {:?}", mac.to_string());
            }
            Ok(None) => {
                eprintln!("No mac address found on {:?}", iface.name);
            }
            Err(e) => {
                eprintln!("Error while getting mac address on {:?}: {:?}", iface.name, e);
            }
        }
    }

    Ok(())
}



////another basic skills for rust and widnows and linux 



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

//another 

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



//another 
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
    let input = String::from("Fukc of"); // বাংলা + ইংরেজি
    match extract_prefix(&input, 5) {
        Some(prefix) => println!("Prefix: {}", prefix),
        None => println!("String too short!"),
    }
}
...



another excercise about encrypt and decrypt a text string using windows dpapi 
//another 
//This is a simpel example ..and for real life security I should use more powerful and letest encryption method 
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



//  the usage of .nth() and .get() 
fn main() {
    let v = vec![10, 20, 30, 40, 50];

    // get() ব্যবহার
    if let Some(value) = v.get(2) {                                        
        println!("get(2) gave: {}", value);
    }

    // nth() ব্যবহার (ইটারেটর থেকে)
    if let Some(value) = v.iter().nth(2) {
        println!("nth(2) gave: {}", value);
    }
}
...........some explanation of the previous code ....
both vector.get()  and .nth() are  Option<i32> and the are a little bit diffirent
vector_veriable.get() this can be used only for vector with option type for Option<&..> types 
but .nth() is very flexible and can work with almost all kind of iterator in rust ....and its also Option<&...> type as .get() method ....but for .nth() method 
it must be an iterator 



//another simple trick that may improve my debugging skills 
fn main(){
    let value = 21 ;
    dbg!(value) ;                            //autometically debug code in order to write more safer code in rust 
}



//basic debugging 
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let p = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    println!("{:?}", p); // Debug print
}

/////////////////////////////Some explanation of previous ..................................
#[derive(Debug)]  it will compile the code autometically in the debug mode .....if i use this i must have to use {:?} to the ptintln .....and i may use dbg!() for more debugging ....in my code
    in my code i should get an error because i did not use println or any perticular reason of this 2 veriable -name and age ......2 way to fix this warning/error 
    and that is use ...println!("{:?} {:?}" , person.name , person.age) ; .........or u may use #[allow(dead_code)] to fuck the warning of rust compiler
    BUT THE BEST PRACTISE IS NOT TO USE #[allow(dead_code)]


example src/main.rs
____________________
#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let person1 = Person {
        name: String::from("Kohee"),
        age: 34,
    };

    println!("{:?}", person1);
}


///another example of vector struct 
#[derive(Debug)] 
struct Log {
    message : Vec<String> ,
}
fn main(){
    let log = Log {
        message : vec![
            "System booted".to_string() , 
            "User logged in".to_string() ,
        ] ,
    }; 
    println!("Your log message {:?}" , log.message) ;
}

////previous code explained --->?
Nothing is new here ....A Struct is like a struct and a vector is like a vector 
Nothing else 



//another 
#[derive(Debug)] 
struct Address{
    city : String , 
    zip : u32 ,
} 

#[derive(Debug)] 
struct Person{
    name : String,
    age :  u32 , 
    address : Address ,
}
fn main(){
    let person = Person{
        name : "Kohee".to_string() , 
        age : 13 , 
        address : Address{
            city : "Dhaka".to_string() , 
            zip : 1023 ,
        } , 
    }; 
    println!("{:#?}" ,person.name);
    println!("{:#?}" ,person.age) ; 
    println!("{:#?}" ,person.address) ; 
    println!("{:#?}" ,person.address.city) ; 
    println!("{:#?}" ,person.address.zip) ; 
}                                                  //this code a little bit diffirent from the previous code ....

//
#[derive(Clone)]   
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1.clone();  // u cannot use clone() without definning #[derive(Clone)]

    println!("({}, {})", p2.x, p2.y);
}


//another example 
#[derive(PartialEq)]        //this line compare the veriable ....maybe equal or not 
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 3, y: 4 };
    let b = Point { x: 3, y: 4 };
    let c = Point { x: 5, y: 6 };

    println!("a == b: {}", a == b); // true
    println!("a == c: {}", a == c); // false
}


//another example of enum using #[derive[PartialEq)] 
#[derive(PartialEq)]
enum Status {
    Online,
    Offline,
    Unknown,
}

fn main() {
    let s1 = Status::Online;
    let s2 = Status::Offline;
    let s3 = Status::Online;

    println!("s1 == s2: {}", s1 == s2); // false
    println!("s1 == s3: {}", s1 == s3); // true
}



//A simple example of box<[u8]>
fn main() {
    let arr: [u8; 4] = [1, 2, 3, 4];
    let boxed: Box<[u8]> = Box::new(arr);     //compiler does not knows the heap allocated size so we have to allocate it 

    println!("Boxed slice: {:?}", boxed);
}


//Simple example of hashmap 
use std::collections::HashMap;

fn main() {
    let mut student_marks = HashMap::new();

    student_marks.insert(String::from("Alice"), 85);
    student_marks.insert(String::from("Bob"), 92);
    student_marks.insert(String::from("Charlie"), 78);

    println!("🎓 Student Marks:");
    for (name, mark) in &student_marks {
        println!("{}: {}", name, mark);
    }
}


//another example 
use std::collections::HashMap;

fn main() {
    let mut capitals = HashMap::new();

    capitals.insert(String::from("Bangladesh"), String::from("Dhaka"));
    capitals.insert(String::from("India"), String::from("New Delhi"));
    capitals.insert(String::from("Japan"), String::from("Tokyo"));

    println!("🌍 Country Capitals:");
    for (country, capital) in &capitals {
        println!("{}: {}", country, capital);
    }
}


//another simple practise of usize 
fn get_nth_element(arr: &[i32], index: usize) -> i32 {
    arr[index]
}

fn main() {
    let array = [5, 10, 15, 20];
    let value = get_nth_element(&array, 2);

    println!("3rd element: {}", value);
}
//another simple example of nth and usize 
fn main() {
    let text = "Kohee 😈";
    let index: usize = 2;

    let ch = text.chars().nth(index);
    println!("Char at index {}: {:?}", index, ch);
}

//another simple example of &&i32 .....double reference 
fn print_value(r: &&i32) {              //the double reference 
    println!("value = {}", **r);        //dereferencing 
}

fn main() {
    let x = 10;
    let r = &x;
    let rr = &r;

    print_value(rr); // rr = &&i32    
}



//An advance code example of multiple use case 
use std::collections::HashSet;

fn main() {
    let mut vector: Vec<i32> = Vec::new();

    for i in 1..100 {
        vector.push(i * 2);
    }

    println!("The vector is {:?}", vector);

    let even_squire = <Vec<_> as std::iter::IntoIterator>::into_iter(vector)
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .collect::<Vec<_>>();

    println!("The even squire is {:?}", even_squire);

    let sum: i32 = even_squire.iter().sum();
    println!("The sum of the even squire is {}", sum);

    let max = even_squire.iter().max().unwrap_or(&0);
    println!("The max of the even squire is {}", max);

    let min = even_squire.iter().min().unwrap_or(&0);
    println!("The min of the even squire is {}", min);

    let average = sum as f64 / even_squire.len() as f64;
    println!("The average of the even squire is {}", average);

    let mut sorted_vector = even_squire.clone();
    sorted_vector.sort();
    println!("The sorted vector is {:?}", sorted_vector);

    let reversed_vector: Vec<_> = sorted_vector.iter().rev().cloned().collect();
    println!("The reversed vector is {:?}", reversed_vector);

    let unique_vector: Vec<_> = sorted_vector
        .iter()
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    println!("The unique vector is {:?}", unique_vector);

    let mut vector_with_duplicates = sorted_vector.clone();
    vector_with_duplicates.push(2);
    vector_with_duplicates.push(4);

    let unique_vector_with_duplicates: Vec<_> = vector_with_duplicates
        .iter()
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    println!(
        "The unique vector with duplicates removed is {:?}",
        unique_vector_with_duplicates
    );
}
