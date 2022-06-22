# Rust By Example Snippets

This repository holds my code snippets as I work my way through the [Rust By Example book](https://doc.rust-lang.org/stable/rust-by-example) to refresh my Rust knowledge. These are mostly reproductions of examples from the book and all credit for the code shown here should go to the authors of the books.

## Organization and Running
This repo is broken down into folders corresponding to the chapters and sections of the book. There is one code file per section of the book. The code files are listed as binary targets in `Cargo.toml` where the target name is the numeric id of the code file with punctuation replaced with underscores. For example, the file at `/1. Hello World/1.2. Formatted Print/1.2.1. Debug.rs` can be run with `cargo run --bin 1_2_1`.
