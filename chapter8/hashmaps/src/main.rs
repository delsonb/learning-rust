use std::collections::HashMap;

fn main() {
    updating_value_based_on_old_value();
}

fn updating_value_based_on_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    //counting words in a sentence
    for word in text.split_whitespace() {
        // if the key isn't in the map, the default value is inserted and, in either case, a mutable reference to the value is returned
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn overwriting_a_value() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);  // score value for Blue team will be overwritten

    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);  // since the entry is already in the map, nothing happens

    println!("{:?}", scores);

}

fn hashmaps_and_ownership() {
    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

fn accessing_values_in_a_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // .get() returns an Option<&V>
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn creating_a_hashmap() {
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

