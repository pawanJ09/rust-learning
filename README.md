# Rust Learning

This project is designed as a quick guide to Rust learning basics. You can run the program or go 
through the main file for details.


## Requirements

For building and running the application you need:

- [Rust](https://www.rust-lang.org/learn/get-started)

Validate version using:

```shell
rustc --version

cargo --version
```


## Running the application locally

You can run the main.rs file to get started if you are not using any IDE. This file has the main 
function.

```shell
cargo build

cargo run
```

## Generate documentation

Documentation comments have been added to the program, but you can generate the Rust documents 
and view them in HTML format using below command.

```shell
cargo doc
```

After generating the documentation you can view it from this path in your browser: target/doc/rust_learning/index.html


## Validate dependencies

You can validate and check the dependencies using

```shell
cargo check
```



