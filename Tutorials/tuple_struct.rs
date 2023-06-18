// tuple_struct.rs: Tuple structs are type aliases for tuple types
fn main() {
    #[derive(Debug)] // For printing
    struct Point(f64, f64);

    // Define a method for Point
    impl Point {
        fn abs(&self) -> f64 {
            let Point(x, y) = self;
            (x * x + y * y).sqrt()
        }
    }

    let p1 = Point(1.0, 2.0);
    let Point(x, y) = p1; // Must use "Point" to deconstruct
    assert_eq!(x, 1.0);
    assert_eq!(y, 2.0);
    println!("{}", p1.abs());

    let p2 = Point { 0: 3.0, ..p1 };
    println!("{:?}", p2);

    let p3 = Point { 1: 4.0, ..p2 };
    println!("{:?}", p3);

    // Can still use .0, .1 ...
    println!("{}", p3.1);
}

/* Output:
2.23606797749979
Point(3.0, 2.0)
Point(3.0, 4.0)
4
*/
