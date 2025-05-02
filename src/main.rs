use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number Game!");

    println!("Enter the minimum value for the range:");
    let mut min_str = String::new();
    io::stdin()
        .read_line(&mut min_str)
        .expect("Failed to read input");

    let min: u32 = match min_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using default value 1.");
            1
        }
    };

    println!("Enter the maximum value for the range:");
    let mut max_str = String::new();
    io::stdin()
        .read_line(&mut max_str)
        .expect("Failed to read input");

    let max: u32 = match max_str.trim().parse() {
        Ok(num) => {
            if num <= min {
                println!(
                    "Maximum must be greater than minimum. Using default value {}.",
                    min + 100
                );
                min + 100
            } else {
                num
            }
        }
        Err(_) => {
            println!("Invalid input. Using default value {}.", min + 100);
            min + 100
        }
    };

    let secret_number = rand::thread_rng().gen_range(min..=max);

    println!("I've picked a number between {} and {}.", min, max);

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
