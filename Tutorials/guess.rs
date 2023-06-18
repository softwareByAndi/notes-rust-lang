// guess.rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let s = loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Assuming no errors here
        io::stdin().read_line(&mut guess).unwrap();
        let guess: u32 = guess.trim().parse().unwrap();
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
