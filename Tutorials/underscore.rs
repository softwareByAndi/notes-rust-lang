fn f<T>(_: T) -> String {
    String::from("hello")
}

fn main() {
    println!("{}", f(1));
    println!("{}", f("one"));
}
