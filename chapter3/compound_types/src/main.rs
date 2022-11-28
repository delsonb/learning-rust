fn main() {
    // Compound types
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);  // fixed size

    let (x, y, z) = tup;  // destructuring (unpacking) a tuple

    println!("The value of y is: {y}");

    let first_element = tup.0;  // period (dot) notation to access elements
    println!("First element of the tuple is: {first_element}");

    let unit = ();  // it represents an empty value or empty return type

    // Array (comma-separated list inside square brackets, all elements have the same type)
    let a = [1, 2, 3, 4, 5];  // fixed size

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];  // number of months is always 12

    let type_annotated_array: [i32; 5] = [1, 2, 3, 4, 5];

    let second_element = a[1];
    println!("2nd element of the array: {second_element}");
    
    let invalid_index: usize = 5;
    // let element = a[invalid_index];  // invalid access causes the program to panic
}
