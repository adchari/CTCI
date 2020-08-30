use std::collections::HashMap;
use std::io::{self, BufRead};

fn check_palindrome(input: &str) -> bool {
    let mut map = HashMap::<char, usize>::new();
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let lower = c.to_ascii_lowercase();
            map.insert(
                lower,
                1 + if map.contains_key(&lower) {
                    map[&lower]
                } else {
                    0
                },
            );
        }
    }

    let mut seen_single = false;
    for (_, v) in map {
        if v % 2 != 0 {
            if seen_single {
                return false;
            }
            seen_single = true;
        }
    }
    true
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", check_palindrome(&line.unwrap()))
    }
}

#[cfg(test)]
mod test {
    use crate::check_palindrome;
    #[test]
    fn example() {
        assert_eq!(true, check_palindrome("Taco Cat"));
        assert_eq!(false, check_palindrome("Adithya Chari"));
    }
}
