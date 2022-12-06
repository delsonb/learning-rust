fn main() {
    using_dbg_macro();
}

// Section: using dbg! macro
#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

fn using_dbg_macro() {
    let scale = 2;
    let rect1 = Rectangle2 {
        width: dbg!(30 * scale),  // dbg! takes ownership of a value and gives is it back
        height: 50,
    };

    dbg!(&rect1); // the line of the debug is also printed
}

// Section: implementing with structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn implementing_with_structs() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle {:?} is {} square pixels",  // "{:?}" debug and "{:#?}" pretty-print directives
        rect1, area3(&rect1)
    );
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Section: implementing a rectangle with tuples

fn implementing_with_tuples() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1  // It isn't very clear which dimension (height or width) is stored on each position
}

// Section: implementing a rectangle

fn implementing_a_rectangle() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}
