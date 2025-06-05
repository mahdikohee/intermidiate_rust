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
