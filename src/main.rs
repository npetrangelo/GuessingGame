mod guess;
use guess::Guess;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Inclusive range from 1 to 100
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret}");
    loop {
        let guess = match Guess::new() {
            Ok(g) => g,
            Err(message) => {
                println!("{}", message);
                continue
            },
        };
        println!("You guessed {guess}");
        match guess.value().cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
