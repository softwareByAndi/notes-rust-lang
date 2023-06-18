// hashmap.rs: Illustrates insert, get, contains_key, entry().or_insert
use std::collections::HashMap;

fn main() {
    let mut nums = HashMap::new(); // Untyped as yet

    nums.insert(1, String::from("one")); // Type now known: [int, String]
    nums.insert(2, String::from("two"));
    println!("{:?}", nums);

    // Access pairs via deconstruction in loop variable
    for (n, s) in &nums {
        println!("{}, {}", n, s);
    }

    // Overwrite with insert
    nums.insert(2, String::from("too"));
    println!("{:?}", nums);

    // Retrieve values
    println!("{}", nums[&1]);
    let found_1 = match nums.get(&1) {
        Some(s) => s,
        None => "Missing",
    };
    println!("{}", found_1);

    if let Some(s) = nums.get(&1) {
        println!("{}", s);
    }

    assert!(!(nums.contains_key(&3)));
    if let Some(s) = nums.get(&3) {
        println!("{}", s);
    } else {
        println!("No such key")
    }

    // Insert only if missing
    println!("{}", nums.entry(3).or_insert(String::from("three")));
    println!("{}", nums[&3]);

    println!("{}", nums.entry(1).or_insert("won".to_string())); // No change
    println!("{:?}", nums);
}

/* Output:
{1: "one", 2: "two"}
1, one
2, two
{1: "one", 2: "too"}
one
one
one
No such key
three
three
one
{2: "too", 3: "three", 1: "one"}
*/
