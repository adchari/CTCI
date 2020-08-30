use std::collections::HashSet;
use std::io::{self, BufRead};

fn all_unique(input: &str) -> bool {
    let mut set = HashSet::<char>::new();
    for c in input.chars() {
        if set.contains(&c) {
            return false;
        }
        set.insert(c);
    }
    true
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", all_unique(&line.unwrap()));
    }
}
