# Exercise - Indexing

In this exercise, modify the code for the two functions. Use the index notation to access the required elements of the numbers tuple and the letters array.

Remember that tuples and arrays have different indexing notation.

You can put the expression for each required element where you see the placeholder todo! macro.

```rust
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    let second = todo!("Replace with the tuple indexing syntax");

    assert_eq!(
        2, second,
        "This is not the 2nd number in the tuple: {}",
        second
    )
}

fn indexing_array() {
    let characters = ['a', 'b', 'c', 'd', 'e'];
    let letter_d = todo!("Replace with the array indexing syntax");

    assert_eq!(
        'd', letter_d,
        "This is not the character for the letter d: {}",
        letter_d
    )
}
```
