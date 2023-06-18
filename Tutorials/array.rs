// array.rs: Loosely based on code in Rust by Example

fn main() {
    let mut array: [i32; 3] = [0; 3]; // Initialize to all zeroes
    assert_eq!([0, 0, 0], array);
    assert_eq!(3, array.len());

    // Overwrite two elements
    array[1] = 1;
    array[2] = 2;
    assert_eq!([1, 2], &array[1..]);

    // Illustrate .get method (returns an Option)
    let a1 = match array.get(1) {
        Some(n) => format!("a[1] == {}", n),
        None => String::from("Bad index!"),
    };
    println!("{}", a1);
    let a100 = match array.get(100) {
        Some(n) => format!("a[100] == {}", n),
        None => String::from("Bad index!"),
    };
    println!("{}", a100);

    // Show "contains" method (must pass a reference)
    println!("{}", array.contains(&2)); // true
    println!("{}", array.contains(&3)); // false

    // 4 ways of printing an array's contents follow...

    // 1) Using the debug formatter
    println!("{:?}", array); // [0, 1, 2]

    // 2) Using an index range (0..=2 == 0..3):
    for i in 0..=2 {
        print!("{} ", array[i]) // 0 1 2
    }
    println!("");

    // 3) Explicitly getting an iterator
    for x in array.iter() {
        print!("{} ", x); // 0 1 2
    }
    println!("");

    // 4) Array references can be coerced into an iterator
    for x in &array {
        print!("{} ", x); // 0 1 2
    }
    println!("");

    // Getting index and element simultaneously (like Python)
    for (i, &item) in array.iter().enumerate() {
        println!("array[{}] == {}", i, &item);
    }

    // Inferring array type from initializer
    let b = [1, 2, 3, 4];
    println!("{:?}", b);

    // Sum b's values (sum requires an iterable and is generic,
    // so we must specify the result type (i32) here).
    let sum: i32 = b.iter().sum();
    println!("{}", sum); // 10
}

/* Output:
a[1] == 1
Bad index!
true
false
[0, 1, 2]
0 1 2
0 1 2
0 1 2
array[0] == 0
array[1] == 1
array[2] == 2
[1, 2, 3, 4]
10
*/
