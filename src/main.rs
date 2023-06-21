mod guess;
use guess::Guess;
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
        if guess.matches(&secret) {
            break;
        }
    }
}
