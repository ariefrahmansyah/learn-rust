fn main() {
    let greeting = String::from("hello");
    let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
    println!("Greeting: {}", greeting); // We can still use `greeting`
}
