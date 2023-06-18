// slice.rs

fn first_word(s: &String) -> &str {
    for (i, item) in s.bytes().enumerate() {
        if item == b' ' {
            return &s[0..i]; // A slice of s; no copying done
        }
    }

    &s // The whole string is 1 word in this case
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("the first word is: {}", word);
}

/* Output:
the first word is: hello
*/
