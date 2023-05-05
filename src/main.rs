use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number {secret_number} ");

    println!("Please input your guessed number: ");

    let mut guess = String::new();

    
    io::stdin().read_line(&mut guess).expect("Failed to read line.");
    
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guessed the number {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small."),
        Ordering::Equal => println!("You win."),
        Ordering::Greater => println!("Too big.")
    }
}
