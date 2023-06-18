#[allow(dead_code)]
pub fn run() {
    test(Some(0));
    test(Some(7));
    test(Some(-1));
    test(None);
}


fn foo(input: Option<i32>) -> Option<i32> {
    input.filter(|x| x >= &0)
    //input.and_then(|x| match x { x if x >= 0 => Some(x), _ => None})
}

fn bar(input: Option<i32>) -> Result<i32, String> {
    foo(input).ok_or("ErrNegative".to_string())
}

fn test(input: Option<i32>) {
    match bar(input) {
        Ok(n) => println!("{}", n),
        Err(s) => println!("{}", s)
    }
}