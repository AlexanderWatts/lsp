use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin().lock();

    for _ in stdin.lines() {
        println!("Hello, world!");
    }
}
