fn main() {
    let s = String::from("book");
    let t = pluralize(&s); // Ownership not transferred
    println!("I have one {}, you have two {}", s, t);
}

fn pluralize(word: &str) -> String {
    word.to_owned() + "s" // Local owned copy (cloned)
}
