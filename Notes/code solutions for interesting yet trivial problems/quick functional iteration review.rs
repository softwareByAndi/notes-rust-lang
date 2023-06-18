fn is_even(n: u32) -> bool {
    n % 2 == 0
}

fn main() {
    let sum_of_even_squares: u32 = (0..)    // infinite loop (only possible because of lazy evaluation)
        .map(|n| n * n)                     // square
        .take_while(|&n| n < 10_000)        // break from loop
        .filter(|&n| is_even(n))            // only take even squares
        .fold(0, |sum, i| sum + i);         // sum all even squares
        
    println!("{}", sum_of_even_squares);
}