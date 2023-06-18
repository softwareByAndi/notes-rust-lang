fn optdiv(num: i32, den: i32) -> Option<i32> {
    if den == 0 {
        None // Or return None;
    } else {
        Some(num / den) // Or return Some(num / den);
    }
}

fn main() {
    println!("{:?}", optdiv(7, 2));
    println!("{:?}", optdiv(7, 0));
}
