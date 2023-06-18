// mymax.rs

fn mymax<T: Ord + Copy>(v: &Vec<T>) -> T {
    assert!(v.len() > 0);
    let mut max = v[0];
    for i in 1..v.len() {
        if v[i] > max {
            max = v[i];
        }
    }
    max
}

fn main() {
    println!("{}", mymax(&vec![1, 2, 3]));
    println!("{}", mymax(&vec!["2", "3", "4"]));
    //    println!("{}", mymax(&vec![String::from("1"), String::from("2")])); //Error!
}

/* Output:error
3
4
*/

/* Error output:
error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
  --> mymax.rs:16:26
   |
3  | fn mymax<T: Ord + Copy>(v: &Vec<T>) -> T {
   |                   ---- required by this bound in `mymax`
...
16 |     println!("{}", mymax(&vec![String::from("1"), String::from("2")]));
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `std::string::String`
*/
