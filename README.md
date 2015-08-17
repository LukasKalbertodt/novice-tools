Rust Tools for Novices
======================
[![Build Status](https://img.shields.io/travis/LukasKalbertodt/novice-tools.svg)](https://travis-ci.org/LukasKalbertodt/novice-tools)
[![License](https://img.shields.io/github/license/LukasKalbertodt/novice-tools.svg)]()
[![Crates.io](https://img.shields.io/crates/v/novice-tools.svg)](https://crates.io/crates/novice-tools)

[**Documentation**](https://lukaskalbertodt.github.io/novice-tools/novice_tools/)

Small and easy functions for absolute beginners in Rust and programming.

Sometimes a seemingly easy task, like reading an integer input from the user, is actually something complex with many sources of errors. Instead of overwhelm the
beginner with the explanation, it's often useful to just start with something easy, like `fn read_number() -> isize`. Easy to use and not very confusing.

*Note*: This means that you shouldn't use this crate in production or something.

## Example
Here is a simple example (see [`example/read_stdin.rs`](https://github.com/LukasKalbertodt/novice-tools/blob/master/examples/read_stdin.rs)):

``` rust
extern crate novice_tools;

use novice_tools::*;

fn main() {
    println!("This was read: {}", read_number("Give me an integer:"));
    println!("This was read: {}", read_bool("Give me a boolean:"));
    println!("This was read: {}", read_float("Give me a float:"));

    let x : usize = read("Give me a positive integer:", "positive integer");
    println!("This was read: {}", x);
}
```
