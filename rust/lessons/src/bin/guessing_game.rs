// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use std::cmp::Ordering;
use std::io;
use std::io::Write;

use rand::Rng;

// Use this API as a contract to guarantee that value will always be in range.
// Note, need mod here because `value` field will be public in this file instead.
mod guess {
    pub struct Guess {
        value: u8,
    }

    impl Guess {
        pub fn new(value: i32) -> Option<Guess> {
            match value {
                0..=100 => Some(Guess { value: value as u8 }),
                _ => None,
            }
        }

        pub fn value(&self) -> u8 {
            self.value
        }
    }
}

fn main() {
    println!("Welcome to Guessing Game!");
    println!("Here you would need to guess a number from 0 to 100!");

    let secret_number: u8 = rand::thread_rng().gen_range(0..=100);

    loop {
        print!("Your number (or `q` to quit): ");
        io::stdout().flush().expect("Unable to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input == "q" {
            break;
        }

        let number: i32 = match input.parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please input a valid number.");
                continue;
            }
        };

        let value = match guess::Guess::new(number) {
            Some(mut guess) => guess.value(),
            None => {
                println!("Guess value must be between 0 and 100, got {}.", number);
                continue;
            }
        };

        match value.cmp(&secret_number) {
            Ordering::Less => println!("Too small.."),
            Ordering::Greater => println!("Too big.."),
            Ordering::Equal => {
                println!("Congratulations! You win the game!");
                break;
            }
        }
    }
}
