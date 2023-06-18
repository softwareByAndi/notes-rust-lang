use std::env;

fn main() {
    let args: Vec<_> = env::args().collect(); // _ infers String
    if args.len() > 1 {
        for s in &args[1..] {
            println!("Hello {}!", s);
        }
    } else {
        println!("Hello, world!");
    }
}
