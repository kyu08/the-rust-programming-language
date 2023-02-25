use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("input your guess!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: i32 = guess.trim().parse().expect("input number");

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("❌less"),
            Ordering::Equal => {
                println!("🙆yes!");
                break;
            }
            Ordering::Greater => println!("❌greater"),
        }
    }
}
