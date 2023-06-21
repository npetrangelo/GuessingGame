use std::cmp::Ordering;
use std::num::ParseIntError;

pub struct Guess {
    value: u32
}

fn input<'a>() -> Result<u32, &'a str> {
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed {guess}");
    return match guess.trim().parse::<u32>() {
        Ok(g) => Ok(g),
        Err(_) => Err("That is not a number!")
    };
}

impl<'a> Guess {
    pub fn new() -> Result<Guess, &'a str> {
        println!("Guess a number:");
        let value = match input() {
            Ok(x) => x,
            Err(E) => {
                return Err(E)
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
