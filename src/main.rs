use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn get_guess(buff: &mut String) -> io::Result<&mut String> {
    print!("Guess a number (1-100): ");

    buff.clear();
    io::stdout().flush()?;
    io::stdin().read_line(buff)?;
    Ok(buff)
}

fn parse_guess(buff: &str) -> Result<u32, &'static str> {
    match buff.trim().parse::<u32>() {
        Ok(num) => Ok(num),
        Err(_) => Err("not a number"),
    }
}

fn eval_guess(guess: u32, secret: u32) -> Result<u32, &'static str> {
    match guess.cmp(&secret) {
        Ordering::Greater => Err("Too high"),
        Ordering::Less => Err("Too low"),
        Ordering::Equal => Ok(guess),
    }
}

fn main() {
    let mut guess = String::new();
    let secret = rand::thread_rng().gen_range(1, 100);

    println!("Welcome to the Guessing Game!");

    loop {
        get_guess(&mut guess).expect("Could not get value from STDIN");

        let guess = match parse_guess(&guess) {
            Ok(num) => num,
            Err(msg) => {
                println!("{} {}", guess.trim(), msg);
                continue;
            }
        };

        match eval_guess(guess, secret) {
            Ok(_) => {
                println!("You guessed correct!");
                break;
            }
            Err(msg) => {
                println!("{}", msg);
                continue;
            }
        };
    }
}
