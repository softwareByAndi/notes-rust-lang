#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of {:?} is {}.", rect1, rect1.area());

    // Default fields from previous struct with ..<name>
    let rect2 = Rectangle { width: 40, ..rect1 };
    println!("The area of {:?} is {}.", rect2, rect2.area());
}

/* Output:
The area of Rectangle { width: 30, height: 50 } is 1500.
The area of Rectangle { width: 40, height: 50 } is 2000.
*/
