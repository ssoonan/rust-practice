use rand::Rng;
use std::io;

fn make_new_number() -> i32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..=100);
    random_number
}

fn main() {
    let computers_number = make_new_number();

    loop {
        let mut input = String::new();
        println!("Enter your number");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let parsed_number = input.trim().parse::<i32>().unwrap();

        if computers_number > parsed_number {
            println!("Your number is smaller than my number!");
            continue
        } else if computers_number < parsed_number {
            println!("Your number is bigger than my number!");
            continue
        } else {
            println!("You're correct! The number was {computers_number}!");
            return;
        }
    }
}
