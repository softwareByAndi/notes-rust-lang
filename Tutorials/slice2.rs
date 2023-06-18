// slice2.rs: Shows the versatility of slice parameters

fn str_slice(s: &str) {
    println!("{} has length {}", s, s.len());
}

fn seq_slice(seq: &[i32]) {
    for elem in seq {
        print!("{:?} ", elem);
    }
    println!("");
}

fn main() {
    str_slice("hello");
    let s = String::from("goodbye");
    str_slice(&s);
    str_slice(&s[..4]);

    let a = [1, 2, 3];
    seq_slice(&a);
    seq_slice(&a[1..]);

    let v = vec![4, 5, 6, 7];
    seq_slice(&v);
    seq_slice(&v[1..3]);
}

/* Output:
hello has length 5
goodbye has length 7
good has length 4
1 2 3
2 3
4 5 6 7
5 6
*/
