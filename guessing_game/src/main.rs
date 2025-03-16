use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please enter your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Please enter a smaller number"),
            Ordering::Equal => {
                println!("Congratulations! You have guessed the correct number");
                break;
            },
            Ordering::Less => println!("Please enter a bigger number"),
        }
    }
}
