// Shows that if let can be used with any enum
#[allow(dead_code)]

#[derive(Debug)]
enum Choices {
    Num(i32),
    Phrase(String),
}
use Choices::*;

fn print_choice(c: &Choices) {
    if let Choices::Num(k) = c {
        println!("{}", k);
    } else {
        println!("{:?}", c);
    }
}
fn main() {
    print_choice(&Num(4));
    print_choice(&Phrase(String::from("four")));

    let mut c = Num(3);
    while let Num(k) = c {
        println!("{}", k);
        c = if k > 0 {Num(k-1)} else {Phrase(String::from("Blast Off!"))};
    }
    if let Phrase(s) = c {
        println!("{}", s);
    }
}

/*
4
Phrase(4, "four")
3
2
1
0
Blast Off!
*/
