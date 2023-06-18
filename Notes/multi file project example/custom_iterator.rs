#[allow(dead_code)]
pub fn run() {
    let mut test_struct = MyStruct::new();
    test_struct.push(1);
    test_struct.push(2);
    test_struct.push(3);
    test_struct.push(4);
    test_struct.push(5);
    test_struct.push("Hello".to_string());
    test_struct.push("world".to_string());
    test_struct.push("my".to_string());
    test_struct.push("name's".to_string());
    test_struct.push("andi".to_string());

    use MyEnum::*;

    for v in test_struct.into_iter() {
        match v {
            I(n) => println!("{}", n),
            S(s) => println!("{}", s),
        };
    }
    for v in test_struct.into_iter() {
        match v {
            I(n) => println!("{}", n),
            S(s) => println!("{}", s),
        };
    }
}

#[derive(Debug, Clone)]
struct MyStruct(Vec<i8>, Vec<String>);

impl MyStruct {
    fn new() -> Self {
        Self(Vec::new() as Vec<i8>, Vec::new() as Vec<String>)
    }
}

trait Append<T> {
    fn push(&mut self, n: T);
}

impl Append<i8> for MyStruct {
    fn push(&mut self, n: i8) {
        self.0.push(n);
    }
}

impl Append<String> for MyStruct {
    fn push(&mut self, s: String) {
        self.1.push(s);
    }
}

#[derive(Debug)]
enum MyEnum<'a> {
    I(&'a i8),
    S(&'a String),
}

impl<'a> IntoIterator for &'a MyStruct {
    type Item = MyEnum<'a>;
    type IntoIter = MyStructIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MyStructIterator {
            data: self,
            index: 0,
        }
    }
}

struct MyStructIterator<'a> {
    data: &'a MyStruct,
    index: usize,
}

impl<'a> Iterator for MyStructIterator<'a> {
    type Item = MyEnum<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.data.0.get(self.index) {
            self.index += 1;
            Some(MyEnum::I(n))
        } else if let Some(s) = self.data.1.get(self.index - self.data.0.len()) {
            self.index += 1;
            Some(MyEnum::S(s))
        } else {
            None
        }
    }
}

// #[derive(Debug, Clone)]
// struct MyStructIterator {
//     data: MyStruct,
//     index: usize,
// }

// #[derive(Debug)]
// enum MyEnum {
//     I(i8),
//     S(String),
// }

// impl IntoIterator for MyStruct {
//     type Item = MyEnum;
//     type IntoIter = MyStructIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         MyStructIterator {
//             data: self,
//             index: 0,
//         }
//     }
// }

// impl Iterator for MyStructIterator {
//     type Item = MyEnum;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(n) = self.data.0.get(self.index) {
//             self.index += 1;
//             Some(MyEnum::I(*n))
//         } else if let Some(s) = self.data.1.get(self.index - self.data.0.len()) {
//             self.index += 1;
//             Some(MyEnum::S(s.clone()))
//         } else {
//             None
//         }
//     }
// }

// ---------------------------------------------------------------------------------
