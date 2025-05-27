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


