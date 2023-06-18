fn main() {
    let s = String::from("hello");
    let t = s;
    println!("{}", t); // hello
//    println!("{}", s); // Error!
}

/* Error Output from line 5:
  |
2 |     let s = String::from("hello");
  |         - move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
3 |     let t = s;
  |             - value moved here
4 |     println!("{}", t);
5 |     println!("{}", s);
  |                    ^ value borrowed here after move
*/

