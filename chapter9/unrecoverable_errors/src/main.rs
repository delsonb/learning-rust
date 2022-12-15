fn main() {
    accessing_invalid_array_positions();
}

fn accessing_invalid_array_positions() {
    let v = vec![1, 2, 3];

    v[99];
}

fn crash_and_burn() {
    panic!("crash and burn");
}
