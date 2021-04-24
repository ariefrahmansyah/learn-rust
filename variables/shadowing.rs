// You can also declare a new variable with the same name as a previous variable, which creates a new binding.
// In Rust, this operation is called "shadowing" because the new variable shadows the previous variable.
// The old variable still exists, but you can't refer to it in this scope anymore.

fn main() {
    let number = 5; // the first binding is created using the name "number"
    let number = number + 5; // a different binding shadows the name "number"
    let number = number * 2; // again, a new binding is created
    println!("The number is: {}", number);
}
