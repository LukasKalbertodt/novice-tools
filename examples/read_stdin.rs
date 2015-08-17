extern crate novice_tools;

use novice_tools::*;

fn main() {
    println!("This was read: {}", read_number("Give me an integer:"));
    println!("This was read: {}", read_bool("Give me a boolean:"));
    println!("This was read: {}", read_float("Give me a float:"));

    let x : usize = read("Give me a positive integer:", "positive integer");
    println!("This was read: {}", x);
}
