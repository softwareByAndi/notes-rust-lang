use std::collections::HashMap;

#[allow(dead_code)]
pub fn run() {
    let vec = vec![0, 1, 2, 3];
    let vec2 = vec![97, 98, 99, 100];
    for (i, v) in vec
        .iter()
        .chain(vec2.iter()) // chain() links 2 iterators together
        .enumerate()
    {
        // enumerate() returns the pair : (index, value)
        println!("{} : {}", i, v);
    }

    // output:
    // 0 : 0
    // 1 : 1
    // 2 : 2
    // 3 : 3
    // 4 : 97
    // 5 : 98
    // 6 : 99
    // 7 : 100

    let vec_c: Vec<_> = vec.iter().map(|x| x * x).collect();
    let map: HashMap<_, _> = vec.iter().map(|x| x * x).enumerate().collect();

    for v in vec_c {
        print!("{},", v);
    }
    println!("");
    // output:
    // 0,1,4,9,2 : 4

    for (i, v) in map {
        println!("{} : {}", i, v);
    }
    // output:
    // 1 : 1
    // 2 : 4
    // 3 : 9
    // 0 : 0

    println!("\n\n\n");

    let mut _iter = (&vec).into_iter();
    while let Some(v) = _iter.next() {
        println!("{}", v);
    }
    println!("---------");
    // task is to rewrite this while let code using loop and match
    let mut _iter = (&vec).into_iter();
    loop {
        match _iter.next() {
            Some(v) => {
                println!("{}", v);
            }
            None => break,
        };
    }

    // output:
    // 0
    // 1
    // 2
    // 3
    // ---------
    // 0
    // 1
    // 2
    // 3
}
