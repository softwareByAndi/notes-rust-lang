fn fib(n: i32) -> i32 {
    assert!(n >= 0);
    if n == 0 || n == 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

// Iterative version
fn fib2(n: i32) -> i32 {
    assert!(n >= 0);
    if n == 0 || n == 1 {
        n
    } else {
        let mut pair = (0, 1);
        for _i in 2..=n {
            pair = (pair.1, pair.0 + pair.1);
        }
        pair.1
    }
}
fn main() {
    for number in 0..=10 {  // Inclusive range
        println!("fib({}) = {}", number, fib(number));
    }

    println!("{}", fib2(10));
}

/* Output:
fib(0) = 0
fib(1) = 1
fib(2) = 1
fib(3) = 2
fib(4) = 3
fib(5) = 5
fib(6) = 8
fib(7) = 13
fib(8) = 21
fib(9) = 34
fib(10) = 55
55
*/
