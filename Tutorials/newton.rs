// newton.rs: Newton's method for sqrt
use std::f64; // For EPSILON

fn my_sqrt(a: f64, mut x0: f64) -> f64 {
    let mut x1 = (x0 + a / x0) / 2.0;
    while (x1 - x0).abs() > f64::EPSILON * x0.abs() {
        let temp = x1;
        x1 = (x1 + a / x1) / 2.0;
        x0 = temp;
    }
    x1
}

fn main() {
    let x: f64 = 2.0;
    print!("Mine: {}, ", my_sqrt(x, 1.0));
    println!("Theirs: {}", x.sqrt());
}

/* Output:
Mine: 1.414213562373095, Theirs: 1.4142135623730951
*/
