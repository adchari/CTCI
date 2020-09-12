#![feature(linked_list_cursors)]

use std::collections::LinkedList;

fn delete_nth<T>(list: &mut LinkedList<T>, n: usize) {
    let mut cursor = list.cursor_front_mut();
    for _ in 0..n {
        cursor.move_next();
    }
    cursor.remove_current();
}

fn main() {
    // Unimplemented
}

#[cfg(test)]
mod test {
    use crate::delete_nth;
    use std::collections::LinkedList;
    #[test]
    fn example() {
        let mut l1: LinkedList<i32> = LinkedList::new();
        l1.push_back(1);
        l1.push_back(2);
        l1.push_back(3);
        l1.push_back(4);
        l1.push_back(5);

        let mut l2: LinkedList<i32> = LinkedList::new();
        l2.push_back(1);
        l2.push_back(2);
        l2.push_back(3);
        l2.push_back(5);

        delete_nth(&mut l1, 3);
        assert_eq!(l2, l1);
    }
}
