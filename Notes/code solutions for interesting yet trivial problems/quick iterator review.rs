fn main() {
    let mut v = vec![1, 2, 3];
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for i in &v {
        println!("{:?}", i);
    }
    for i in &mut v {
        *i = *i * 10;
    }
    println!("{:?}", v);
    
    
    let mut iter = v.into_iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    
    // println!("{}", v); // <-- throws error here



    // .iter() does not consume the variable
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    // .iter_mut() does not consume the variable
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}