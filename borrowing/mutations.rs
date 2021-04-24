fn change_immutable(message: &String) {
    // We try to add a "!" to the end of our message
    // message.push_str("!");

    // But, this code will not compile
    //
    //  error[E0596]: cannot borrow `*message` as mutable, as it is behind a `&` reference
    //  --> mutations.rs:2:5
    //   |
    // 1 | fn change_immutable(message: &String) {
    //   |                              ------- help: consider changing this to be a mutable reference: `&mut String`
    // 2 |     message.push_str("!"); // We try to add a "!" to the end of our message
    //   |     ^^^^^^^ `message` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

fn change_mutable(text: &mut String) {
    text.push_str(", world");
}

fn main() {
    let greeting_immutable = String::from("Hello");
    change_immutable(&greeting_immutable);

    let mut greeting_mutable = String::from("Hello");
    change_mutable(&mut greeting_mutable);
}
