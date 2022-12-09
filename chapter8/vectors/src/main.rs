fn main() {
    iterating_over_vectors();
}

fn holding_different_types_in_a_vector() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn iterating_over_vectors() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;  // * is the derefence operator and it must be used to modify the value pointed by the mutable reference
    }
    println!("{:?}", v);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // v.push(*i); // NOT allowed: the for loop holds a mutable reference to the vector, a second one cannot exist simultaneously
    }
}

fn borrow_rules_with_vectors() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];  // immutable reference

    v.push(6);  // push() requires a mutable borrow. Not allowed because first is referenced after this point

    // Uncomment to get a compiler error message
    // println!("The first element is: {}", first);
}

fn reading_from_vectors() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];  // trying to access an invalid position this way will cause the program to panic
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
}

fn vector_data_type() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 4];  // type is inferred automatically
    let mut v3 = Vec::new();

    v3.push(5);  // the first usage tells the compiler which data type the vector holds
    v3.push(6);
    v3.push(7);
    v3.push(8);
}
