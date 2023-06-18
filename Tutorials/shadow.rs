fn main() {
    let x = 3;
    let addx = |a| a + x; // Closure (lambda)

    // Shadowing
    let x = 10;
    println!("{}", addx(2));
    println!("{}", x);

    // Shadowing again!
    let x = "hello";
    println!("{}", x);
}

/* Output:
5
10
hello
*/
