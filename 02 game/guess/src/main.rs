use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("## Guess the Number ##");

    let secret_number: i32 = rand::rng().random_range(1..=100);

    loop {
        println!("enter a number");
        let mut guess: String = String::new(); // return a instance of String
        io::stdin() // it append the new string into it
            .read_line(&mut guess) // mutable reference
            .expect("Error while reading");
        // while reading string from terminal it takes while input like <ourInput>/n a extra new line chacter too

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            }
        }; // shadowing old guess to new guess

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won");
                break;
            }
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
        }
    }
    println!("Thanks for Playing");
}
