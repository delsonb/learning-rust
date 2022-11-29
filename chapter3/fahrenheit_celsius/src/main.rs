use std::io;

fn main() {
    let mut fahrenheit = String::new();

    println!("Please type a temperature in fahrenheit degrees: ");
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Not a valid temperature! Exiting...");
    
    let celsius = (fahrenheit - 32.0) / 1.8;

    println!("\n{fahrenheit:.1} °F equals {celsius:.1} °C."); // formatted printing
}
