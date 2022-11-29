fn main() {
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

fn move() {
    let x = 5;
    let y = x; // integers are simple types, so a copy is made and two 5`s are pushed onto the stack

    // with String something different happens
    let s1 = String::from("hello");
    let s2 = s1;
     
}
