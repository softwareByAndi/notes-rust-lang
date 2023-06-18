fn isprime(n: i32) -> bool {
    fn hasdiv(num: i32, mut div: i32) -> bool {
        while div * div <= num {
            if num % div == 0 {
                return true;
            }
            div += 1;
        }
        false
    }
    n >= 2 && !hasdiv(n, 2)
}

fn main() {
    for i in 1..10 {
        if isprime(i) {
            println!("{}", i);
        }
    }
}

/* Output:
2
3
5
7
*/
