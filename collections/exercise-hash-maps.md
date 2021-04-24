# Exercise - Hash maps

In this exercise, you'll need to define a basket of fruit in the form of a hash map. The key represents the name of the fruit. The value represents how many of that particular fruit are in the basket.

You have to put at least three different types of fruit in the basket. For example, three types might be apple, banana, and mango. The total count of all the fruits should be at least five.

You only need to edit the fruit_basket function in this exercise.

```rust
use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = todo!("declare your hash map here.");

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.

    basket
}

fn main() {
    let basket = fruit_basket();
    assert!(
        basket.len() >= 3,
        "basket must have at least three types of fruits"
    );
    assert!(
        basket.values().sum::<u32>() >= 5,
        "basket must have at least five fruits"
    );
}
```
