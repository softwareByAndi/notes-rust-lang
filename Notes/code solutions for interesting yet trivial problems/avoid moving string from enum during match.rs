#[allow(unused_variables)]
#[allow(dead_code)]

#[derive(Debug)]
enum TestStruct {
    TS1(i64, String),
    TS2(f32, char)
}

fn main() {
    let _x = TestStruct::TS2(10., 'a');
    let _x = TestStruct::TS1(10, String::from("hello world"));
    
    let a = match _x {
                // use the ref keyword here to avoid move
        TestStruct::TS1(i, ref s) => {
            (i as f64, String::from(s))
        }
        TestStruct::TS2(f, c) => {
            (f as f64, c.to_string())
        }
    };
    
    println!("{:?} : {}", a.0, a.1);
    println!("{:?}", _x);
    
    // String copies
    let s1 = String::from("original");
    let mut s2 = String::from(&s1);
    s2.push_str("_copy()");
    
    println!("{}  |  {}", s1, s2);
}

    // OUTPUT

    // 10.0 : hello world
    // TS1(10, "hello world")
    // original  |  original_copy()