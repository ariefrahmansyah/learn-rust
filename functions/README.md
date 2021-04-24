# Functions

Functions are the primary way code is executed within Rust. You've already seen one of the most important functions in the language. The `main` function is the entry point of many programs.

Function definitions in Rust start with `fn` and have a set of parentheses after the function name. The braces tell the compiler where the function body begins and ends.

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Hello from another function!");
}
```

A function can be called by entering its name followed by a set of parentheses, passing any arguments as necessary. In the preceding example, `another_function` required no arguments, so we haven't passed any.

We defined `another_function` after the `main` function in the source code. We could have defined it before too. Rust doesn't care where in your file you define your functions, only that they're defined somewhere in your file.

## Pass parameters to functions

In the following example, we're going to declare a function that checks if a given number is divisible by another and returns a `boolean` value to confirm that.

```rust
fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
  // If the divisor is zero, we want to return early with a `false` value
  if divisor == 0 {
    return false;
  }
  dividend % divisor == 0
}
```

Let's take a look at this function signature:

- `fn`: The function declaration keyword in Rust.
- `is_divisible_by`: The function name.
- `(dividend: u32, divisor: u32)`: This function's parameter list. It states that two unsigned 32-bit integers are expected as input values.
- `-> bool`: The arrow points to the type of value this function will always return.

The `is_divisible_by` function accepts two integers as inputs and outputs a boolean value.

Now let's look closer at this function's body:

```rust
if divisor == 0 {
    return false;
}
```

All this part of the function is trying to do is prevent a classic programming error, the division by zero error.

The last line in our function's body is an expression without the `return` keyword:

```rust
dividend % divisor == 0
```

In Rust, the last expression inside a code block (`{ ... }`) is always returned, so we don't need to use the return `keyword` in here.

This expression uses the remainder operator (`%`) to get the remainder of the division between the two terms and compare it to zero. The resulting type after applying the equality operator (`==`) is the `bool` type, which is either `true` or `false`.

Call a function
Let's see our function in action.

```rust
fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
  if divisor == 0 {
    return false;
  }
  dividend % divisor == 0
}

fn main() {
  assert_eq!(is_divisible_by(2, 3), false);
  assert_eq!(is_divisible_by(5, 1), true);
  assert_eq!(is_divisible_by(24, 6), true);
}
```
