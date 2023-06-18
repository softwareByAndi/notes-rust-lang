// chars.rs
fn main() {
    let s = String::from("Löwe 老虎 Léopard Gepardi");
    println!("{}", s.len()); // # of bytes
                             // s.bytes().count()
    println!("{}", s.chars().count()); // # of complete characters

    // chars() returns an iterator
    for c in s.chars() {
        print!("{}", c);
    }
    println!();

    // bytes() returns an iterator
    for b in s.bytes() {
        println!("{}", b);
    }
    println!("{}", &s[1..3]); // Prints ö (195|182)
                              // println!("{}", &s[1..2]); // Error
}

/* Output:
29
23
Löwe 老虎 Léopard Gepardi
76
195
182
119
101
32
232
128
129
232
153
142
32
76
195
169
111
112
97
114
100
32
71
101
112
97
114
100
105
ö
*/

/* Runtime error output:
thread 'main' panicked at 'byte index 2 is not a char boundary;
it is inside 'ö' (bytes 1..3) of `Löwe 老虎 Léopard Gepardi`
*/
