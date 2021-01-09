use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the Number!");
    
    

    loop {
        let mut guess = String::new();
        print!("Input your guess: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut guess)
            .expect("Failed to recieve guess!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small"),
            Ordering::Equal => {println!("You win!"); break;},
            Ordering::Greater => println!("To big"),
        }
    }
    
}
