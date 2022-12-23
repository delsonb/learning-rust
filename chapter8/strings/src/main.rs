fn main() {
    methods_for_iterating_over_strings();
}

fn methods_for_iterating_over_strings() {
    // char
    println!("Iterating over chars");
    for c in "Здр".chars() {  // each char represents a unicode point
        println!("{c}");
    }

    // raw bytes
    for b in "Здр".bytes() {
        println!("{b}");  // valid unicode scalar values may be made up of more than 1 byte
    }

    // A case where a unicode point doesn't correspond to a 'letter'
    for c in "नमस्ते".chars() {
        println!("{c}");  // there are 6 chars, but two of them are diacritics
    }
    // To get what we perceive as letters, we should iterate over "grapheme clusters" (not available in the standard library)
}

fn indexing_strings() {
    let hello = String::from("Здравствуйте");
    // let h = s1[0]; // indexing with integers is not allowed

    let s1 = &hello[0..4]; // create a slice with the first 4 bytes, not characters
    println!("{s1}"); // it prints "Зд"

    // Panic if we try to slice the string disrespecting the char boundaries
    // let s2 = &hello[0..3];  // Uncomment to see it panicking
}

fn concatenating_with_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format!() doesn't take ownership of any of the parameters
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}

fn concatenating_multiple_strings() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");
}

fn concatenating_strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    // After the following operation, s1 is moved, but s2 is not
    let s3 = s1 + &s2;
    println!("{s3}");
}

fn updating_a_string() {
    let mut s = String::from("foo");
    // pushing a string slice
    s.push_str("bar");
    // push_str() doesn't take ownership of the parameter
    let s1 = String::from("bar");
    s.push_str(&s1);
    println!("s1 is still valid here: {}", s1);

    //pushing a char with push()
    let mut s2 = String::from("lo");
    s2.push('l');
}

fn string_besides_ascii() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn creating_a_string_from_literal() {
    let s = "string literal".to_string();
    println!("{}", s);
}
