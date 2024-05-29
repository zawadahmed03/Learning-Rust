use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");
        let secret_number = rand::thread_rng().gen_range(1..=10);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, the actual number is {secret_number}"),
            Ordering::Greater => println!("Too big, the actual number is {secret_number}"),
            Ordering::Equal => {
                println!("You win!");
                break;
                }
            }
    
        }
    }
