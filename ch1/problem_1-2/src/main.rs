use std::io::{self, BufRead};

fn is_permutation(i1: &str, i2: &str) -> bool {
    let mut i1: Vec<char> = i1.chars().collect::<Vec<char>>();
    i1.sort();
    let mut i1 = i1.iter();
    let mut i2: Vec<char> = i2.chars().collect::<Vec<char>>();
    i2.sort();
    let mut i2 = i2.iter();

    loop {
        match (i1.next(), i2.next()) {
            (Some(x), Some(y)) => {
                if x != y {
                    return false;
                }
            }
            (None, None) => return true,
            _ => return false,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let str1 = iterator.next().unwrap().unwrap();
    let str2 = iterator.next().unwrap().unwrap();
    println!("{}", is_permutation(&str1, &str2));
}

#[cfg(test)]
mod test {
    use crate::is_permutation;
    #[test]
    fn not_perm() {
        assert_eq!(false, is_permutation("abcd", "efgh"));
        assert_eq!(false, is_permutation("abcd  ", "abcd"));
    }
    #[test]
    fn perm() {
        assert_eq!(true, is_permutation("abcd", "cdab"));
        assert_eq!(true, is_permutation("abcd  ", " a bcd"));
    }
}
