// some functionalities for ramdon number and comparisons
use rand::Rng;
use std::cmp::Ordering;
// for reading an input
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guesses = 1;
    // iterate over and over until the guess is correct
    loop {
        println!("Please enter your guess number (between 1 - 100): ");
        let mut guess_number = String::new();

        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read line");

        let guess_number: usize = guess_number
            .trim()
            .parse()
            .expect("Your guess was not a number");

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => break
        }
        guesses += 1;
    }
    println!("Congrats, you guessed the secret number => {secret_number}. You took {guesses} guesses to discover!");
}
