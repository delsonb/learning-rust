fn main() {
   loops_with_labels();
   returning_from_a_loop();
   repetition_with_loop_basic();
   ternary_operator();
   chained_if_and_else();
   if_and_else();
   repetition_with_while();
   looping_through_a_collection_without_for();
   looping_through_a_collection_with_for();
   counting_down_with_for_loop();
}

fn counting_down_with_for_loop() {
    // This is the equivalent of Python range(n)
    // rev() reverse the collection
    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn looping_through_a_collection_with_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn looping_through_a_collection_without_for() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // error prone and slow
    // compiler adds runtime code to check whether the index is within the bounds of the array
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn repetition_with_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loops_with_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // By default, break and continue apply to the innermost loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn returning_from_a_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // The break will work as a return
        }
    };

    println!("The result is {result}");
}

fn repetition_with_loop_basic() {
    let mut counter = 5;

    loop {
        println!("again!");

        counter -= 1;
        if counter <= 0 {
            break;
        }
    }
}

fn ternary_operator() {
    let condition = true;
    let number = if condition {5} else {6};
    
    println!("The value of number is: {number}");
}

fn chained_if_and_else() {
    let number = 6;

    // This snippet can be refactored using "match" [Chapter 6]
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_and_else() {
    let number = 7;

    if number > 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Unlike Python and Ruby, Rust will not try to convert non-Boolean types to a Boolean automatically
    // if number { } WRONG
    if number != 0 {
        println!("number was something other than zero!");
    }


}
