// bunch2.rs: Makes sum a constrained method

use std::{clone::Clone, iter::Sum};

#[derive(Debug)]
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
    fn sum(&self) -> T
    where
        T: Clone + Sum<T>, // Required traits for this method (Sum is value-based, not &)
    {
        match &self {
            One(x) => x.clone(),                 // Because x is a reference
            Group(v) => v.iter().cloned().sum(), // Ditto v
        }
    }
}

fn main() {
    let b1 = One(1);
    println!("{:?} (size = {:?})", b1, b1.size());
    let b2 = Group(vec![1.0, 2.0]);
    println!("{:?}", b2.sum());
}

/* Output:
One(1) (size = 1)
3.0
*/
