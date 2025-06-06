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



