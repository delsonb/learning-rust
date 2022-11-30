fn main() {
    using_mutable_references();
}

// Section: having multiple references
fn 

// Section: reference basics
fn using_mutable_references() {
    let mut s = String::from("hello");

    add_world_to_hello(&mut s);  // the reference must be marked as mutable

    println!("{s}");
}

fn add_world_to_hello(s: &mut String) {  // the function parameter must be marked as mutable reference too
    s.push_str(", wolrd!");
}

fn using_references() {
    let s1 = String::from("hello");

    let len = calculate_length_with_reference(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_with_reference(s: &String) -> usize {
    s.len()
}
