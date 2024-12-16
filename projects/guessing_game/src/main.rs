use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        println!("Enter a guess!");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

        println!("The secret number is {secret_number}, you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!")
        };
    }
}
