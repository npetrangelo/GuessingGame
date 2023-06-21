use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

pub struct Guess {
    value: u32
}

fn input() -> Result<u32, ParseIntError> {
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");
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

impl Display for Guess {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
