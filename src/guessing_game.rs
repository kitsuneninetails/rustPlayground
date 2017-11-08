extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn guessing_game() {
    println!("Guess a number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed read");

        let guess: u32 = match guess.trim().parse() {
            Result::Ok(n) => n,
            Result::Err(e) => {println!("ERROR: {}", e); 0},
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Low!"),
            Ordering::Greater => println!("High"),
            Ordering::Equal => {
                println!("CORRECT!");
                break;
            },
        }
    }
}
