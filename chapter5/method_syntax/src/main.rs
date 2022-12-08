
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {  // &self is a shorthand notation for self: &Self
        self.width * self.height
    }

    fn set_height(&mut self, height: u32) {  // In this method, the instance is borrowed mutably
        self.height = height;
    }

    fn width(&self) -> bool {  // a method can have the same name as one of the struct's fields
        self.width > 0         // and it is not necessarily a getter method
    }

    fn can_hold(&self, rect: &Self) -> bool {  // the second parameter could have been written as rect: &Rectangle
        self.width >= rect.width && self.height >= rect.height 
    }

    fn square(size: u32) -> Self {  // without a &self first parameter, we are defining associated functions that don't require a instance
        Self {                      // they are equivalent to Python's classmethods and can be used to define constructors
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 60,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    rect1.set_height(50);
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    if rect1.width() {  // the syntax determines if we are calling the method or the field named "width"
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Creating a square from a "classmethod"
    let square = Rectangle::square(50);  // to call the associated function "square", we use :: syntax
    println!(
        "This rectangle is a square {}x{}",
        square.width, square.height
    );
}
