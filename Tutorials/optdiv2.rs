// optdiv2.rs: Illustrates "if let"
fn optdiv(num: i32, den: i32) -> Option<i32> {
    if den == 0 {
        None // Or return None;
    } else {
        Some(num / den)
    }
}

fn main() {
    println!("{:?}", optdiv(7, 2));
    println!("{:?}", optdiv(7, 0));

    // Use pattern matching, a la ML
    match optdiv(7, 2) {
        Some(i) => {
            println!("The integer is {}", i);
        }
        None => {
            println!("There is no integer");
        }
    }

    // Another way with "if let"
    // Syntax: if let <pattern> = <expression {<body>}
    if let Some(i) = optdiv(7, 2) {
        println!("The integer is {}", i);
    } else {
        println!("There is no integer");
    }

    // If you know the pattern matches, you can leave off the else
    if let None = optdiv(7, 0) {
        println!("There is no integer");
    }
}

/* Output:
Some(3)
None
The integer is 3
The integer is 3
There is no integer
*/
