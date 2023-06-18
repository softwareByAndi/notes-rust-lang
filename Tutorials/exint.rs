// exint.rs

#[derive(Debug)] // For printing with {:?}
enum Exint {
    // datatype Exint = Value of int | PlusInf | MinusInf;
    Value(i32),
    PlusInf,
    MinusInf,
}
use Exint::*; // To avoid Exint:: below

impl Exint {
    fn square(&self) -> Exint {
        match &self {
            Value(n) => Value(n * n),
            PlusInf => PlusInf,
            MinusInf => PlusInf,
        }
    }
}

fn main() {
    let (x, y, z) = (Value(5), PlusInf, MinusInf);
    println!("{:?}, {:?}, {:?}", x.square(), y.square(), z.square());
}

/* Output:
Value(25), PlusInf, PlusInf
*/
