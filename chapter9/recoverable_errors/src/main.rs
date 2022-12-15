use std::fs::File;
use std::io::ErrorKind;

fn main() {
    handling_result_without_match();
}

fn handling_with_expect() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt must be included in this project");  // expect() lets you define a custom error message

}

fn handling_with_unwrap() {
    let greeting_file = File::open("hello.txt").unwrap();  // the panic! macro is called automatically
}


fn handling_result_without_match() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello_created.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn opening_an_inexistent_file_v2() {
    // return Result<T, E>, where T -> type of success value (a file handle); E -> std::io::Error
    let greeting_file_result = File::open("hello.txt");

    // It's possible to match different kinds of errors
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello_created.txt") {  // Create if not exists
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),

            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

fn opening_an_inexistent_file() {
    // return Result<T, E>, where T -> type of success value (a file handle); E -> std::io::Error
    let greeting_file_result = File::open("hello.txt");

    // One way to handle the Result<T, E> enum is using match
    // Like the Option<T> enum, the variants Result::Ok and Result::Err are brought into scope by the prelude
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}