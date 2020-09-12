use std::collections::LinkedList;
fn kth_from_end<T: Clone>(list: &LinkedList<T>, k: usize) -> Option<&T> {
    let mut runner = list.iter().peekable();
    let mut walker = list.iter();
    for _ in 0..=k {
        runner.next();
    }

    if let None = runner.peek() {
        return None;
    }

    loop {
        match runner.next() {
            None => match walker.next() {
                None => return None,
                Some(x) => return Some(x),
            },
            Some(_) => {
                walker.next();
            }
        }
    }
}

fn main() {
    // Unimplemented
}

#[cfg(test)]
mod test {
    use crate::kth_from_end;
    use std::collections::LinkedList;
    #[test]
    fn example() {
        let mut l1: LinkedList<i32> = LinkedList::new();
        l1.push_back(1);
        l1.push_back(2);
        l1.push_back(3);
        l1.push_back(4);
        l1.push_back(5);

        assert_eq!(Some(&2), kth_from_end(&l1, 3));
        assert_eq!(None, kth_from_end(&l1, 5));
        assert_eq!(Some(&5), kth_from_end(&l1, 0));
    }
}
