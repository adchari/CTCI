use std::io::{self, BufRead};

fn compressed(i1: &str) -> String {
    let mut s = String::new();
    s.reserve(i1.len());

    let mut it = i1.chars();
    let mut c: char = it.next().unwrap();
    let mut count = 1;

    for x in it {
        if x != c {
            s.push_str(&format!("{}{}", c, count));
            c = x;
            count = 1;
        } else {
            count = count + 1;
        }
    }
    s.push_str(&format!("{}{}", c, count));

    if i1.len() <= s.len() {
        String::from(i1)
    } else {
        s
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line1 = iterator.next().unwrap().unwrap();
    println!("{}", compressed(&line1));
}

#[cfg(test)]
mod test {
    use crate::compressed;
    #[test]
    fn example() {
        assert_eq!(String::from("a2b1c5a3"), compressed("aabcccccaaa"));
        assert_eq!(String::from("abcdef"), compressed("abcdef"));
        assert_eq!(String::from("aabbccddeeff"), compressed("aabbccddeeff"));
        assert_eq!(String::from("a10"), compressed("aaaaaaaaaa"));
    }
}
