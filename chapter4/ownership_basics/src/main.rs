/*
  Rust rules of ownership:
    - Each value in Rust has an owner.
    - There can only be one owner at a time.
    - When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    return_values_and_scope();
}

// Section: Returning ownership of parameters
fn returning_multiple_values_to_take_ownership_back() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

// Section: Return values and scope
fn return_values_and_scope() {
    let s1 = gives_ownership();  // gives ownership moves its return value into s1

    let s2 = String::from("hello");  // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back,
                                        // which also moves its return value into s3

    println!("{s2}");
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // a_string is returned and moves out to the calling function
}

// Section: Ownership and functions
fn onwership_and_functions() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    println!("s is no longer valid: {}", s); // compile-time error!

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 implements Copy, so it's okay to still
                                    // use x afterward
    println!("x is still valid: {}", x);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


// Section: ways variables and data interact
fn clone_variables() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deeply copy the heap data (potentially expensive operation!)

    println!("s1 = {}, s2 = {}", s1, s2);  // both s1 and s2 are valid here
}

fn moving_ownership() {
    let x = 5;
    let y = x; // integers are simple types, so a copy is made and two 5`s are pushed onto the stack

    // with String something different happens
    let s1 = String::from("hello");

    println!("{s1}"); // This works

    let s2 = s1;  // After this point, s1 will no longer be valid
    
    // println!("{s1}");  // value borrowed after move - compiler error
}

fn understanding_scope() {
    let s = "hello"; // string literal stored on the stack

    // the String type is stored on the heap
    // here, we create a String from a string literal
    let s = String::from("hello");

    // string literals are immutable, but the String type can be mutated
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    { // beginning of a new scope
        let greeting = String::from("hello");  // greeting is valid from this point foward

        // do stuff with greeting, like printing or appending content
    } // this scope is now over, and s is no longer valid
    // At this point, Rust automatically calls the function `drop`
}
