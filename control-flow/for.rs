fn main() {
    let a = [10, 20, 30, 40, 50];

    // Using iterator
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Using range notation
    for item in 0..5 {
        println!("{}", item * 2);
    }
}
