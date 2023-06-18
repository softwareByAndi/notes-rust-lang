fn main() {
    let mut s = String::from("book");
    print!("I have one {}, you have two ", s);
    pluralize(&mut s);
    println!("{}", s);
}

fn pluralize(word: &mut String) {
    word.push_str("s");
}
