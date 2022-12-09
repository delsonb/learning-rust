fn main() {
    exhaustive_if_let();
}

fn exhaustive_if_let() {
    let mut config_max: Option<u8> = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}.", max);
    } else {
        println!("The maximum is not configured.");
    }

    config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}.", max);
    } else {
        println!("The maximum is not configured.");
    }
}

fn using_if_let() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {  // if let is NOT exhaustive!
        println!("The maximum is configured to be {}", max);
    }
}

// if let syntax applies to the case the only one pattern matters
// and we want to ignore the rest
fn without_if_let() {
    let config_max = Some(3u8);
    match config_max {  // we only want to process one variant
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
