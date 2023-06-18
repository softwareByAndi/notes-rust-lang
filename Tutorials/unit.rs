fn foo() {} // Same as fn foo() -> () {}

fn main() {
    assert!(foo() == ());
}
