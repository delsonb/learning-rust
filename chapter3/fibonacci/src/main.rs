use std::io;

fn main() {
    println!("How many Fibonacci numbers you would like to generate?");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read user input!");

    let n: i32 = n.trim().parse().expect("Invalid number. Input must be an integer!");

    let mut fib_n_2: i64 = 1;
    let mut fib_n_1: i64 = 0;

    for i in 1..=n {
        let fib_n = fib_n_1 + fib_n_2;
        println!("{i}: {fib_n}");
        fib_n_2 = fib_n_1;
        fib_n_1 = fib_n;
    }
}
