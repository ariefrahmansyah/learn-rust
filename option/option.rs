fn main() {
    // Vec::get method returns Option type
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);

    // Pattern matching
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    // `if let` expression
    let some_number: Option<u8> = Some(7);
    match some_number {
        Some(7) => println!("That's my lucky number!"),
        _ => {},
    }

    if let Some(7) = some_number {
        println!("That's my lucky number!");
    }
}
