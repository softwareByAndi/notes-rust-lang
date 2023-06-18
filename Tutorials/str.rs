// str.cpp: Illustrates the str type

// str is a String "slice", a "view" into UTF8 bytes
// in memory. Internally it holds a pointer and a length.
//
// Quoted literals are of type immutable &str with
// static storage duration.
//
// Strings can be coerced to &str, so a &String can be passed.

fn print_str(s: &str) {
    println!("{}", s);
}

fn main() {
    print_str("hello");
    let s = String::from("goodbye");
    print_str(&s); // Coerced to &str
    print_str(&s[..4]); // These work because...
    print_str(&s[4..]); // the strings are ASCII.
    print_str(&s[..]);
}

/* Output:
hello
goodbye
good
bye
goodbye
*/
