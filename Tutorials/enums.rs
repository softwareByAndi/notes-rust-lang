// Simple enum with value type
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

In ML:

datatype IpAddr = V4 of string | v6 of string;
let home = V4 "127.0.0.1";
let loopback = V6 "::1";

#![allow(unused_variables)]
fn main() {
    enum Option<T> {
        Some(T),
        None,
    }
}

// Option
In ML: datatype 'a option = NONE | SOME of 'a;

fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}

More succinct way with if let:

fn main() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

