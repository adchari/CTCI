use std::io::{self, BufRead};

fn is_one_away(i1: &str, i2: &str) -> bool {
    if isize::abs(i1.len() as isize - i2.len() as isize) > 1 {
        return false;
    }

    let v1: Vec<char> = i1.chars().collect();
    let v2: Vec<char> = i2.chars().collect();
    let mut changed = false;
    let mut i = 0;
    let mut j = 0;
    while i < v1.len() && j < v2.len() {
        if v1[i] != v2[j] && changed {
            return false;
        } else if v1[i] != v2[j] {
            if v1.len() > v2.len() {
                j -= 1;
            } else if v1.len() < v2.len() {
                i -= 1;
            }
            changed = true;
        }

        i += 1;
        j += 1;
    }
    true
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line1 = iterator.next().unwrap().unwrap();
    let line2 = iterator.next().unwrap().unwrap();
    println!("{}", is_one_away(&line1, &line2));
}

#[cfg(test)]
mod test {
    use crate::is_one_away;
    #[test]
    fn example() {
        assert_eq!(true, is_one_away("pale", "ple"));
        assert_eq!(true, is_one_away("pales", "pale"));
        assert_eq!(true, is_one_away("pale", "bale"));
        assert_eq!(false, is_one_away("pale", "bake"));
    }
}
