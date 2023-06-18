// newton3.rs: Uses loop instead of while
use std::f64; // For EPSILON

fn my_sqrt(a: f64, x0: f64) -> f64 {
    let mut x = (x0, (x0 + a / x0) / 2.0);
    loop {
        if (x.1 - x.0).abs() <= f64::EPSILON * x.1.abs() {
            break x.1;
        }
        x = (x.1, (x.1 + a / x.1) / 2.0);
    }
}

fn main() {
    let x: f64 = 2.0;
    print!("Mine: {}, ", my_sqrt(x, 1.0));
    println!("Theirs: {}", x.sqrt());
}

/* Output:
Mine: 1.414213562373095, Theirs: 1.4142135623730951
*/
