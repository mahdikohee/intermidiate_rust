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
