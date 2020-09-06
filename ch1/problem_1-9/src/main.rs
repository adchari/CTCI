use std::io::{self, BufRead};

fn rotate_eq(key: String, rotated: String) -> bool {
    let mut r2 = rotated.clone();
    r2.push_str(&rotated);
    r2.contains(&key)
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line1 = iterator.next().unwrap().unwrap();
    let line2 = iterator.next().unwrap().unwrap();
    println!("{}", rotate_eq(String::from(line1), String::from(line2)));
}

#[cfg(test)]
mod test {
    use crate::rotate_eq;
    #[test]
    fn example() {
        assert_eq!(
            true,
            rotate_eq(String::from("waterbottle"), String::from("erbottlewat"))
        );
    }
}
