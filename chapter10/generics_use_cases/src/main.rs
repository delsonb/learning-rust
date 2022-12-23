fn main() {
    mixing_multiple_generic_types();
}

// Section: generics in struct definitions

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct PointMixed<T, U> {
    x: T,
    y: U,
}

fn testing_structs() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 2.0 };
    // let mixed_types = Point {x: 1, y: 2.0}; // NOT allowed
    let integer_and_float = PointMixed { x: 1, y: 2.0 };

    println!("Point with integer coordinates: {:?}", integer);
    println!("Point with float coordinates: {:?}", float);
    println!("Point with mixed type coordinates: {:?}", integer_and_float);
}

// Section: generics in enum definitions

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T), // T represents a valid value type, e.g. std::fs::File
    Err(E),  // E represents an error type, e.g. std::io::Error
}

// Section: generics in method definitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// A method can be implemented only for a specific type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn calling_methods_with_generics() {
    let p = Point { x: 5.0, y: 10.0 };

    println!("p.x = {}", p.x());
    // distance_from_origin can't be called for Point<i32> for example
    println!("Distance from origin: {}", p.distance_from_origin()); 
}

// Section: using different types in method definitions for a struct with generics
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {  // X1 and Y1 are relavant to the struct definition
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {  // X2 and Y2 are only relevant to the method definition
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn mixing_multiple_generic_types() {
    // In this example
    // X1: i32, Y1: f64, X2: String and Y2: char
    let p1 = Point2 { x: 5, y: 10.4};
    let p2 = Point2 { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
