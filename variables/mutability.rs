// In Rust, variable bindings are immutable by default.
// When a variable is immutable, after a value is bound to a name, you can't change that value.
//
// To mutate a value, we must first use the mut keyword to make a variable binding mutable instead.

fn main() {
    let mut mutable_number = 10;
    mutable_number = 20;
    println!("the mutable number is {}.", mutable_number);

    let immutable_number = 100;
    immutable_number = 200; // error[E0384]: cannot assign twice to immutable variable `immutable_number`
}
