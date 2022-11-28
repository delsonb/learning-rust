use std::io;  // Bring io input/output library into scope
use std::cmp::Ordering;
use rand::Rng; // external crate

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);  // immutable variable (default)

    loop {
        println!("PLease input your guess");

        let mut guess = String::new();  // mutable variable 
    
        io::stdin()  // io::stdin() returns an instance of std::io::Stdin which is a handle to the standard input 
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {  // matches the expression with the content of the first arm
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            },
        }
    }
}
