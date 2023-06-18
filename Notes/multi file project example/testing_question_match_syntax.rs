#[allow(dead_code)]
pub fn run() {
    let v = vec![
        "TEST TEST".to_string(),
        "TEST YOU".to_string(),
        "TEST ME".to_string(),
        "Mwahahaha!".to_string(),
    ];
    for i in v {
        match test(i) {
            Err(s) => println!("{}", s),
            _ => continue,
        };
    }
}

struct Foo {
    value: String,
}

impl Foo {
    fn new(value: String) -> Self {
        Self { value: value }
    }

    fn foo(self) -> Result<Foo, String> {
        let v = self.value;
        match v {
            v if v.contains("ME") => Ok(Foo::new(String::from("FOUND 'TEST ME'!"))),
            v if v.contains("YOU") => Ok(Foo::new(String::from("FOUND 'TEST YOU'!"))),
            _ => Err(format!("ERROR in Foo  : self.value = {}", v)),
        }
    }
}

fn h(i: String) -> Result<Foo, String> {
    match i {
        i if i.contains("TEST") => Ok(Foo::new(i)),
        _ => Err(format!("ERROR in h : input = '{}'", i)),
    }
}

// let r = h()?;
//      is the same as saying :
// let i = match h() {
//     Ok(i) => i,
//     err => return err
// };

fn test(i: String) -> Result<String, String> {
    let r = h(i)?.foo()?.value;

    println!("SUCCESS! : r = {}", r);
    Ok(r)
}
