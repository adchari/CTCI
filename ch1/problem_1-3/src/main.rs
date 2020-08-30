use std::io::{self, BufRead};

fn replace_spaces(input: &str) -> String {
    let mut url = String::with_capacity(input.len());
    for c in input.chars() {
        if c == ' ' {
            url.push_str("%20");
        } else {
            url.push(c);
        }
    }
    return url;
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line = iterator.next().unwrap().unwrap();
    println!("{}", replace_spaces(&line));
}

#[cfg(test)]
mod test {
    use crate::replace_spaces;
    #[test]
    fn example() {
        assert_eq!(
            String::from("Mr%20John%20Smith"),
            replace_spaces("Mr John Smith")
        );
        assert_eq!(String::from("BobJoe"), replace_spaces("BobJoe"));
    }
}
