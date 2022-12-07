use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("{}", "==Guessing Game==".blue());
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The Secret Number: {}", secret_number);
    loop {
        println!("{}", "Enter your guess: ".blue());
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot Read Line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Equal => {
                println!("{}", "You Won!".green());
                break;
            }
        }
    }
}
