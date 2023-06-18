// move.rs: Shows move semantics of String (which implements the Drop trait)
fn announce(n: i32) {
    println!("Behold: {}!", n);
}

fn announce_s(s: String) {
    println!("Behold: {}!", s);
}

fn announce_s_give_back(s: String) -> String {
    println!("Behold: {}!", s);
    s
}

fn announce_s_borrow(s: &String) {
    println!("Behold: {}!", s);
}

fn main() {
    // ints are copied
    let n = 7;
    println!("Passing {}", n);
    announce(n);
    println!("{} was passed\n", n);

    // Strings are moved
    let s = String::from("seven");
    println!("Passing \"{}\"", s);
    announce_s(s);
    // println!("{} was passed", s); // Error: s was "moved"
    println!();

    // Copy back (gives ownership back to local s here)
    let mut s = String::from("eight");
    s = announce_s_give_back(s);
    println!("{} was passed\n", s);

    // Could force pass-by-value with clone
    println!("Passing a clone of \"{}\"", s);
    announce_s(s.clone());
    println!("\"{}\" is still here!\n", s);

    // Borrow via a reference
    let t = String::from("nine");
    println!("Passing \"{}\"", t);
    announce_s_borrow(&t);
    println!("\"{}\" was passed", t);
}

/* Output:
Passing 7
Behold: 7!
7 was passed

Passing "seven"
Behold: seven!

Behold: eight!
eight was passed

Passing a clone of "eight"
Behold: eight!
"eight" is still here!

Passing "nine"
Behold: nine!
"nine" was passed
*/

/* Error when line 30 is uncommented:
error[E0382]: borrow of moved value: `s`
  --> move.rs:30:31
   |
27 |     let s = String::from("seven");
   |         - move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
28 |     println!("Passing \"{}\"", s);
29 |     announce_s(s);
   |                - value moved here
30 |     println!("{} was passed", s); // Error: s was "moved"
   |                               ^ value borrowed here after move
*/
