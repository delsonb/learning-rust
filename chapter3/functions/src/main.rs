fn main() {
    another_function(return_five());
    print_labeled_measuments(plus_one(return_five()), 'h');
}

fn another_function(x: i32) {  // we MUST declare the type of each parameter
    println!("The value of x is: {x}");
}

fn print_labeled_measuments(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn return_five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
