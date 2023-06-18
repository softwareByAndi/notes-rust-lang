// vec.rs: Illustrates push, pop, get, explicit dereferencing

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}, len == {}", v, v.len());

    // Indexing
    println!("v[2] == {}", v[2]);
    println!("v.get(2) == {:?}", v.get(2));
    println!("v.get(10) == {:?}", v.get(10));
    let mut n = match v.get(2) {
        Some(k) => k,
        None => &(-1),  // note the reference to the literal
    };
    println!("n == {}", n);
    n = match v.get(10) {
        Some(k) => k,
        None => &(-1),
    };
    println!("n == {}", n);

    // Using if let instead (can be used with any enum)
    if let Some(k) = v.get(2) {
        println!("n == {}", k);
    } else {
        println!("n == -1");
    }
    if let Some(k) = v.get(10) {
        println!("n == {}", k);
    } else {
        println!("n == -1");
    }

    // Add 1 to all elements of v (by dereferencing)
    // note the mut keyword
    for n in &mut v {
        *n += 1;
    }
    println!("{:?}", v);

    // Illustrate "while let" to empty the vector
    while let Some(k) = v.pop() {
        println!("{}", k);
    }
}

/* Output:
[1, 2, 3, 4], len == 4
v[2] == 3
v.get(2) == Some(3)
v.get(10) == None
n == 3
n == -1
n == 3
n == -1
[2, 3, 4, 5]
5
4
3
2
*/
