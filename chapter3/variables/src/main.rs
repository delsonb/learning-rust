const SECONDS_PER_DAY: u32 = 60 * 60 * 3;  // constants are always immutable and MUST by annotated

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;  // immutable
    println!("The value of y is: {y}");
    let y = 6;  // still immutable
    println!("The variable y was shadowed and now has a new value: {y}");
    let y = "six";
    println!("Shadowing allows us to change the type a variable reusing the name: {y}");
}
