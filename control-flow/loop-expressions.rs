fn main() {
    let mut i = 1;
    let something = loop {
        i *= 2;
        if i > 100 {
            break i;
        }
    };
    println!("{}", something);
}
