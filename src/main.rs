use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number!");

    loop {
        println!("Input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = guess.trim().parse().expect("Please enter a number.");

        println!("You guessed: {}", guess);

        if guess < secret_number {
            println!("More");
        } else if guess > secret_number {
            println!("Less");
        } else {
            println!("Congratulations! You guessed the number.");
            break;
        }
    }

    println!("The secret number was: {}", secret_number);
}
