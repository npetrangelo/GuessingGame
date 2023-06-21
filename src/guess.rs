use std::cmp::Ordering;
use std::num::ParseIntError;

pub struct Guess {
    value: u32
}

fn input() -> Result<u32, ParseIntError> {
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed {guess}");
    return guess.trim().parse::<u32>();
}

impl<'a> Guess {
    pub fn new() -> Result<Guess, &'a str> {
        println!("Guess a number:");
        let value = match input() {
            Ok(x) => x,
            Err(_) => {
                return Err("That is not a number!")
            },
        };
        match value {
            1..=100 => Ok(Guess { value }),
            _ => {
                Err("Number must be between 1 and 100")
            }
        }
    }

    pub fn matches(&self, other: &u32) -> bool {
        match self.value.cmp(other) {
            Ordering::Less => {
                println!("Too small!");
                false
            }
            Ordering::Equal => {
                println!("You win!");
                true
            },
            Ordering::Greater => {
                println!("Too big!");
                false
            }
        }
    }
}
