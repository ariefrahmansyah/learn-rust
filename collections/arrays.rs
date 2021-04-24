fn main() {
    // a comma-separated list inside of brackets
    let weekdays = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    println!("{:?}", weekdays);

    // initialize an array of 512 elements where every element is a zero
    let byte_buffer = [0_u8; 512];
    println!("{:?}", byte_buffer);

    // you can access elements of an array by using indexing, which starts at 0
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    println!("first element of the array: {}", letters[0]); // prints 'a'
    println!("second element of the array: {}", letters[1]); // prints 'b'
}
