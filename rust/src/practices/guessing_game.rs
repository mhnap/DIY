// https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html

use std::cmp::Ordering;
use std::io;
use std::io::Write;

use rand::Rng;

fn main() {
    println!("Welcome to Guessing Game!");
    println!("Here you would need to guess a number from 0 to 100!");

    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        print!("Your number: ");
        io::stdout().flush().expect("Unable to flush stdout");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small.."),
            Ordering::Greater => println!("Too big.."),
            Ordering::Equal => {
                println!("Congratulations! You win the game!");
                break;
            }
        }
    }
}
