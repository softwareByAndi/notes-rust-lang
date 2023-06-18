// tuple.rs

fn main() {
    let barney = (1+2, 3.0*4.0, "brown");
    println!("{:?}", barney);
    println!("{}", barney.0);

    let (a, b, c) = barney;
    println!("{},{:.2},{}", a, b, c);

    let point1 = ("red", (300,200));
    println!("{}", (point1.1).1) // == point1.1.1
}

/* Output:1(3, 12.0, "brown")
3
3,12.00,brown
200
*/
