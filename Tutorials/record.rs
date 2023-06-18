// record.rs: Illustrates structs as data records

fn main() {
    struct Person {
        age: u8,
        name: String,
    }

    let p = Person {
        age: 39,
        name: String::from("bill"),
    };
    println!("{} is {}", p.name, p.age); // bill is 39
}
