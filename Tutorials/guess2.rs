// guess2.rs: Shows error handling through matching on Result
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let s = loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Check for input errors (process returned Result)
        io::stdin().read_line(&mut guess).expect("Read error!");
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                break "You win!";
            }
        }
    };
    println!("{}", s);
}
