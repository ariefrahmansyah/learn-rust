# Fatal errors with panic!

Panicking is the simplest error handling mechanism in Rust.

You can use the `panic!` macro to panic the current thread. It prints an error message, unwinds and cleans up the stack, and then exits the program.

Let’s try calling the `panic!` macro in a simple program:

fn main() {
    panic!("At The Disco");
}

This program would exit with status code *101* and print the following message:

```
thread 'main' panicked at 'At The Disco', panic.rs:2:5
```

The preceding panic message reveals the place in the source code where the panic occurred, `panic.rs:2:5`. The message indicates that it’s the fifth character on the second line of the `panic.rs` file.

In general terms, you should use `panic!` when a program reaches an unrecoverable state meaning anything where there is absolutely no way to recover from the error.

Rust panics on some operations such as a division by zero or an attempt to access an index that isn't present in an array, a vector, or a hash map, as shown in the following code:

```rust
let v = vec![0, 1, 2, 3];
println!("{}", v[6]); // this will cause a panic!
```
