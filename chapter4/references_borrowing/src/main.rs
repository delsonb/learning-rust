/*
  Rust rules of references:
    - At any given time, you can have either one mutable reference or any number of immutable references.
    - References must always be valid.
*/

fn main() {
    println!("Hello, World!");
}

// Section: dangling references
// fn creating_dangling_references() {
//     let dangling = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     // &s // try to return a reference to some to a value owned by this function and whose ownership is not transfered
// }

// Section: having multiple references

fn combining_mutable_and_immutable_references_v2() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem, because multiple immutable refereces are allowed

    println!("{} and {}", r1, r2);

    let r3 = &mut s; // No problem, because r1 and r2 are not used after this point

    println!("{}", r3);
}

fn combining_mutable_and_immutable_references() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem, because multiple immutable refereces are allowed
    let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3); // At this point r1 and r2 believe s was unchanged, but r3 could have destroyed that belief
                                            // that is why Rust forbids it
}

fn mutable_references_in_different_scopes() {
    fn main() {
        let mut s = String::from("hello");
    
        {
            let r1 = &mut s;
        } // r1 goes out of scope here, so we can make a new reference with no problems.
    
        let r2 = &mut s;  // as r1 went out of scope, r2 is the only mutable reference to s at this point
    }
    
}

fn trying_to_create_two_mutable_references() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; // a second mutable reference is forbidden...

    // println!("{}, {}", r1, r2);  // ...because both of them should be valid here -> this prevents data races
}

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
