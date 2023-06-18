// Just testing transfer of ownership. Would nomally use a reference.
fn main() {
    let mut s = String::from("Food");
    s = modify_and_return(s);
    modify(&mut s);
    println!("{}",s);
}

fn modify_and_return(mut s: String) -> String {
    s.push_str("!");
    s       // Or return s;
}

fn modify(s: &mut String) {
    s.push_str("!");
}