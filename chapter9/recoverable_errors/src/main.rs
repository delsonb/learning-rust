use std::fs::File;
use std::error::Error;
use std::io::ErrorKind;
use std::io::{self, Read};

// main() can return () or Result<(), E>
fn main() -> Result((), Box<dyn Error>){  // Box<dyn Error> is a trait object, meaning "any kind of error"
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

fn using_question_mark_when_returning_option(text: &str) -> Option<char> {
    // return last char of the first line
    text.lines().next()?.chars().last()
}

fn even_even_shorter_read_username_from_file -> Result<String, io::Error> {
    fs::read_to_string("hello.txt");
}

fn even_shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// The question mark operator allows us to propagate errors more easily
fn short_read_username_from_file() -> Result<String, io::Error> {
    // if OK(_), the value inside the Ok is returned, else the error is returned as if we had used "return"
    // ? automatically tries to use "from" to convert the type of the error to the return type of our function 
    let mut username_file = File::open("hello.txt")?;  
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// By getting the error back, the calling code can decide to call panic!, to use a default
// username or get the username from somewhere other than a file
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = file::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
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