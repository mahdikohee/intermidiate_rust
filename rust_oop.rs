//I still suck at Rust OOP.
//A simple example of rust oop
//ex-1 
trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    let num1: i32 = 42;
    let num2: f64 = 3.14;

    println!("{}", num1.show()); // Output: four-byte signed 42
    println!("{}", num2.show()); // Output: eight-byte float 3.14
}

//ex2
// Trait definition
trait Greet {
    fn greet(&self);
}

// Struct
struct Person {
    name: String,
}

// Trait implement for struct
impl Greet for Person {
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}

// main function to run
fn main() {
    let person = Person {
        name: String::from("Rust Learner"),
    };
    person.greet();
}

//another oop practise on self 
struct Hacker {
    name: String,
    level: u32,
}

impl Hacker {
    // Constructor function using Self
    fn new(name: &str, level: u32) -> Self {
        Self {
            name: name.to_string(),
            level,
        }
    }

    fn display(&self) {
        println!("Name: {}, Level: {}", self.name, self.level);
    }
}

fn main() {
    let dd = Hacker::new("Hacker DD", 99);
    dd.display();
}

//lets learn generics 
use std::fmt::Debug;

fn print_value<T: Debug>(value: T) {
    println!("The value is {:?}", value);
}

fn main() {
    print_value(21);
    print_value("Hello world");
    print_value(3.33);
}



//another example 
use std::fmt::Debug;

fn print_value<T: Debug>(value: T) {
    println!("The value is {:?}", value);
}

fn main() {
    print_value(21);
    print_value("Hello world");
    print_value(3.33);
}


//another example 
#[derive(Debug)]
struct Container<T> {
    item: T,
}

fn main() {
    let int_box = Container { item: 42 };
    let str_box = Container { item: "Rust" };
    
    println!("{:?}", int_box);
    println!("{:?}", str_box);
}


//another example
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

fn main() {
    let p1 = Point::new(3, 4);
    let p2 = Point::new(2.5, 6.7);
    
    println!("{:?}", p1);
    println!("{:?}", p2);
}


///another generics example which returns something 
fn return_first<T, U>(a: T, _b: U) -> T {
    a
}

fn main() {
    let result = return_first("Hello", 42);
    println!("{}", result);
}


///another oop example 
// Show trait ডিফাইন করলাম
trait Show {
    fn show(&self) -> String;
}

// i32 এর জন্য Show ইমপ্লিমেন্ট করলাম
impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

// f64 এর জন্য Show ইমপ্লিমেন্ট করলাম
impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    let answer: i32 = 42;
    let maybe_pi: f64 = 3.14;

    // Trait Object: &dyn Show হলো 'Trait Object' টাইপ
    let v: Vec<&dyn Show> = vec![&answer, &maybe_pi];

    for d in v.iter() {
        // এখানে Runtime এ সঠিক Show::show() কল হয়
        println!("show {}", d.show());
    }
}

//another simple example 
trait Show {
    fn area(&self) -> f64;
}

struct Circle {
    redious: f32,
}

impl Show for Circle {
    fn area(&self) -> f64 {
        (self.redious as f64) * 3.1416 * 3.1416
    }
}

struct Rectangle {
    length: i32,
    width: i32,
}

impl Show for Rectangle {
    fn area(&self) -> f64 {
        (self.length * self.width) as f64
    }
}

fn main() {
    let cir = Circle { redious: 32.323 };
    let rec = Rectangle { length: 32, width: 10 };
    let vector: Vec<&dyn Show> = vec![&cir, &rec];
    for i in vector.iter() {
        println!("Area {:?}", i.area());
    }
}




///another example 
use std::fmt;

#[derive(Debug)] // এটি Debug ট্রেট অটোমেটিক যোগ করে
struct Person {
    name: String,
    age: u8,
}

// এখন Display ট্রেট নিজে ইমপ্লিমেন্ট করি
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} এর বয়স {} বছর", self.name, self.age)
    }
}

fn main() {
    let p = Person {
        name: "Ayon".to_string(),
        age: 23,
    };

    // Debug প্রিন্ট (ডেভেলপমেন্টের জন্য)
    println!("Debug আউটপুট: {:?}", p);

    // Display প্রিন্ট (ইউজারের জন্য)
    println!("Display আউটপুট: {}", p);
}


//another example 
use std::fmt::Display;

fn displays_it<T: Display>(input: T) {
    println!("{}", input);
}

fn main() {
    displays_it("Hacker Dad");
    displays_it(666);
}



//another oop based example for rust 
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct Errorone;

impl Error for Errorone {}
impl fmt::Display for Errorone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got your first error @")
    }
}

#[derive(Debug)]
struct Errortwo;                // ← struct ডিক্লেয়ার করি

impl Error for Errortwo {}
impl fmt::Display for Errortwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got your second error !")
    }
}

fn return_error(input: u8) -> Result<String, Box<dyn Error>> {
    match input {
        0 => Err(Box::new(Errorone)),
        1 => Err(Box::new(Errortwo)),
        _ => Ok("looks fine to me !".to_string()),  // ← to_string()
    }
}

fn main() {
    let vector: Vec<u8> = vec![1, 2, 3];
    for number in vector {
        match return_error(number) {               // ← number, not &number
            Ok(msg)  => println!("Your number is {:?}", msg),
            Err(err) => eprintln!("Error as e {:?}", err),
        }
    }
}


