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

//Another simple example of explicit rust 
fn maybe_get_number(get_it: std::primitive::bool) -> std::option::Option<std::primitive::i32> {
    if get_it {
        std::option::Option::Some(42i32)
    } else {
        std::option::Option::None
    }
}

fn main() {
    let result: std::option::Option<std::primitive::i32> =
        maybe_get_number(std::primitive::bool::from(true));

    match result {
        std::option::Option::Some(n) => {
            let stdout: std::io::Stdout = std::io::stdout();
            let mut handle: std::io::StdoutLock<'_> = stdout.lock();
            use std::io::Write;

            let bytes: &[u8] = {
                // Manually build message: "Got a number: 42\n"
                // We convert the number to string explicitly
                let mut buffer: std::string::String = std::string::String::from("Got a number: ");
                use std::fmt::Write as FmtWrite;
                let _ = std::fmt::write(&mut buffer, std::format_args!("{}", n));
                let _ = buffer.push('\n');
                buffer.as_bytes()
            };

            let _ = handle.write_all(bytes);
        }
        std::option::Option::None => {
            let stdout: std::io::Stdout = std::io::stdout();
            let mut handle: std::io::StdoutLock<'_> = stdout.lock();
            use std::io::Write;

            let _ = handle.write_all(b"Got nothing\n");
        }
    }
}
