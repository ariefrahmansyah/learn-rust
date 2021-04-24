# Hello World with Cargo

## What is Cargo?

While the `rustc` compiler is fine for simple programs, almost no projects use the Rust compiler directly. Instead they use Rust's build tool and dependency manager, Cargo.

When you install `rustup`, you'll also get the latest stable version of Cargo.

Cargo does lots of things for you, including:

- Creating new project templates with `cargo new`.
- Building your project with `cargo build`.
- Building and running your project with `cargo run`.
- Testing your project with `cargo test`.
- Checking your project type checks with `cargo check`.
- Building documentation for your project with `cargo doc`.
- Publishing a library to crates.io with `cargo publish`.

## Create a new project

To create a new project:

```
$ cargo new hello-cargo
```

To initialize a new project on existing directory:

```
$ cargo init
```

The `hello-cargo` directory contains following files:

```
hello-cargo
|- Cargo.toml
|- src
  |- main.rs
```

Cargo.toml is the manifest file for Rust. It's where you keep metadata for your project as well as dependencies.

src/main.rs is where you'll write your application code. You can see that cargo new generated a boilerplate "Hello, world!" project for us.

## Build and run your program with Cargo

```
$ cd hello-cargo
$ cargo run
```

You should see this output in your terminal:

```
   Compiling hello-cargo v0.1.0 (/Users/ariefrahmansyah/go/src/github.com/ariefrahmansyah/hello-rust/hello-cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.12s
     Running `target/debug/hello-cargo`
Hello, world!
```
