struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("The {} email is {}.", user1.username, user1.email);

    println!("Changing the email...");
    user1.email = String::from("anotheremail@example.com");
    println!("The {} email now is {}.", user1.username, user1.email);
}

fn build_user(email: String, username: String) {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_shorthand(email: String, username: String) {
    User {
        email,  // The variable and the struct field have the same name, we can write only "email" instead of "email: email"
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn creating_an_instance_from_another_one() {
    let user1 = build_user(String::from("useremail@example.com"), String::from("username"));

    let user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1  // fields not explicitly set in user2 should have the same value as the fields in user1
    };  // NOTE: the data in user1 was moved to user2
}

// Section: Using Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Section: unit-like structs
struct AlwaysEqual;  // unit like struct is a struct with no fields

fn instantiating_unit_like_structs() {
    let subject = AlwaysEqual;  // no need for parentheses or curly brackets
}

// Section: Ownership of struct data
struct UserReference {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn testing_ownership_in_structs() {
    let user1 = User {
        email: "someone@example.com",  // This code won't compile, because we need lifetime parameters
        username: "someusername123",   // to ensure that the data referenced by the struct is valid for as long as the struct is.
        active: true,
        sign_in_count: 1,
    }; 
}

