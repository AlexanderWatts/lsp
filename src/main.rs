use std::io::{self, BufRead};
mod encode_message;

fn main() {
    let stdin = io::stdin().lock();

    for _ in stdin.lines() {
        println!("Hello, world!");
    }
}
