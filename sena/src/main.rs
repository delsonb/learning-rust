use rand::Rng;

fn main() {
    let mut numbers = Vec::new();

    loop {
        let luck_number = rand::thread_rng().gen_range(1..=60);

        if  !numbers.contains(&luck_number) {
            numbers.push(luck_number);
        }

        if numbers.len() == 6 {
            break;
        }
    }

    println!("Os números da sorte são: {:?}", numbers);
    
}
