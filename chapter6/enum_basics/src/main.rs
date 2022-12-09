fn main() {
    using_option();
}

// Section: the Option enum
fn using_option() {
    let some_number = Some(5);
    let some_char = Some('e');

    // The type must be annotated because the compile can't infer only by looking at a None
    let absent_number: Option<i32> = None;

    // Try to use a Option<T> as T results in a compiler error
    let x: i8 = 5;
    let y: Option<i8> = Some(6);

    let sum = x + y;  // Option<i8> and i8 can't be added!!!
}

// Section: defining methods
enum Message {
    Quit,  // no data
    Move { x: i32, y: i32 },  // named fields like a struct
    Write(String),  // a single string
    ChangeColor(i32, i32, i32),  // three i32 values
}

impl Message {
    fn call(&self) {
        //method body would be defined here
    }
}

fn calling_enum_methods() {
    let m = Message::Write(String::from("hello"));
    m.call();
}


// Section: holding data inside an enum value
enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),  // each variant can have different types and amounts of associated data
    V6(String),
}

fn testing_enums_with_data() {
    let home1 = IpAddr::V4(String::from("127.0.0.1"));
    let home2 = IpAddr2::V4(127, 0, 0, 1);

    let loopback1 = IpAddr::V6(String::from("::1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));
}



// Section: creating Enums
enum IpAddrKind {
    V4,
    V6,
}

fn testing_enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Both values are of the same type and we can define a function that takes any IpAddrKind
    route(IpAddrKind::V4);   
    route(IpAddrKind::V6);
}


fn route(ip_kind: IpAddrKind) {}
