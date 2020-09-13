use std::collections::LinkedList;
fn palindrome<T>(l: LinkedList<T>) -> bool {
    let mut forward = l.iter();
    let mut backward = l.iter().rev();
    // With raw LinkedList this would be a pointer check on a doubly linked list, singly linked may require reversal

    loop {
        match (forward.next(), backward.next()) {
            (Some(x), Some(y)) => {
                if x != y {
                    return false;
                }
            }
            _ => break,
        }
    }

    return true;
}

fn main() {
    // Unimplemented
}

#[cfg(test)]
mod test {
    use crate::palindrome;
    use std::collections::LinkedList;
    #[test]
    fn example() {
        let mut l1: LinkedList<i32> = LinkedList::new();
        l1.push_back(7);
        l1.push_back(1);
        l1.push_back(7);

        let mut l2: LinkedList<i32> = LinkedList::new();
        l2.push_back(5);
        l2.push_back(9);
        l2.push_back(2);
        l2.push_back(1);

        let mut l3: LinkedList<i32> = LinkedList::new();
        l3.push_back(2);
        l3.push_back(1);
        l3.push_back(1);
        l3.push_back(2);
        assert_eq!(true, palindrome(l1));
        assert_eq!(false, palindrome(l2));
        assert_eq!(true, palindrome(l3));
    }
}
