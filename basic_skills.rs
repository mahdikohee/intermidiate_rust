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


