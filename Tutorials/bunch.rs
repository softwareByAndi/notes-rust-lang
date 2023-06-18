// bunch.rs: The sum function below is monomorphic
//
// datatype 'a bunch = One of 'a | Group of 'a list;
enum Bunch<T> {
    One(T),
    Group(Vec<T>),
}
use Bunch::*;

impl<T> Bunch<T> {
    fn size(&self) -> usize {
        match &self {
            One(_) => 1,
            Group(v) => v.len(),
        }
    }
    
    fn sum(b: &Bunch<i32>) -> i32 {
        match b {
            One(x) => x.clone(), // Because x is a reference
            Group(v) => v.iter().sum(),
        }
    }
    
    fn sum(&self) -> T
    where
        T: Clone + Sum<T>, // Required traits for this method (Sum is value-based, not &)
    {
        match &self {
            One(x) => x.clone(),                    // because x is a reference
            Group(v) => v.iter().cloned().sum(),    // Ditto v
        }
    }
}



fn main() {
    println!("{}", One(1.0).size());
    println!("{}", Group(vec![true, false]).size());
    println!("{}", sum(&Group(vec![3, 4, 5])));
}

/* Output:
1
2
12
*/
