fn print_greeting(message: &String) {
    println!("Greeting: {}", message);
}

fn main() {
    let greeting = String::from("Hello");
    print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
    print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again
}
