fn main() {
    corrupting_string_after_slice_was_taken();
}

fn improving_first_word_signature(s: &str) -> &str {  // this way we can use this function with &String and &str

}

fn understanding_string_literals() {
    let s = "Hello, World!";  // the type of literals is &str with a pointer pointing to the binary -> that`s why they are immutable
}

fn corrupting_string_after_slice_was_taken() {
    let mut s = String::from("hi from my program!");

    let word = first_word_with_slices(&s);

    // s.clear(); // this action requires a mutable borrow, but word, that is a immutable reference, is used later. 
                  //So this is FORBIDDEN by reference rules

    println!("{}", word);
}

fn first_word_with_slices(s: &String) -> &str {  // &str = string slice type
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn using_range_syntax() {
    let s = String::from("hello");

    let slice = &s[..2]; // equivalent to &s[0..2]

    let slice = &s[3..]; // includes the last byte of the String

    let len = s.len();
    let slice = &s[0..len]; // take a slice of the entire string
    let slice = &s[..]; // this is not equivalent to &s!

    // NOTE: range indices must occur at valid UTF-8 character boundaries, otherwise the program will exit within an error!!
}

fn slices_basics() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];  // internally Rust stores a pointer to the byte at index 6 and a length value of 11 - 6 = 5.
}

fn first_word_v1(s: &String) -> usize {  // The index returned in this case is not tied to s 
    let bytes = s.as_bytes();            // there is no guarantee that they will be kept in sync

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
