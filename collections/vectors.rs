fn main() {
    let three_numbers = vec![1, 2, 3];
    println!("Initial vector: {:?}", three_numbers); // prints "[1, 2, 3]"

    // the vec! macro also accepts the same syntax as the array constructor
    let ten_zeroes = vec![0; 10];
    println!("Ten zeroes: {:?}", ten_zeroes); // prints [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    // create vector using `Vec::new()`
    let mut v = Vec::new(); // creates an empty vector,
    v.push(5); // pushes the number five into it...
    v.push(6); // ... an then six, and so on
    v.push(7);
    v.push(8);
    println!("{:?}", v); // prints [5, 6, 7, 8]

    // popping values
    let mut v = vec![1, 2];
    let two = v.pop();
    println!("{:?}", two);

    // vectors also support indexing
    let mut v = vec![1, 2, 3];
    let three = v[2];
    v[1] = v[1] + 5;
    println!("{:?}", three);
}
