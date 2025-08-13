use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("======== Welcome to Guess the number! ========");
    let secret_number = rand::thread_rng().gen_range(1..=100);    
        
    loop {
        println!("[+] Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("[!] Failed to read line");

        // In this case the guess is a "shadow variable"
        let guess: u32 = match guess.trim().parse(){
            Ok(n) => n,
            Err(_) => {
                println!("[!] The input must be a number!");
                continue;
            }
        };
        println!("[+] You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("[+] Too small!"),
            Ordering::Greater => println!("[+] Too big!"),
            Ordering::Equal => {
                println!("[+] You win!");
                break;
            }
        }
    }
}
