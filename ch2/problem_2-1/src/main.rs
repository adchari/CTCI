use std::collections::{HashSet, LinkedList};
use std::hash::Hash;

fn remove_duplicates<T: Eq + Hash + Copy>(list: LinkedList<T>) -> LinkedList<T> {
    let mut map: HashSet<T> = HashSet::new();
    let mut unique: LinkedList<T> = LinkedList::new();
    for it in list.iter() {
        if !map.contains(it) {
            unique.push_back(*it);
            map.insert(*it);
        }
    }
    return unique;
}

fn main() {
    // Unimplemented
}

#[cfg(test)]
mod test {
    use crate::remove_duplicates;
    use std::collections::LinkedList;
    #[test]
    fn example() {
        let mut l1: LinkedList<i32> = LinkedList::new();
        let mut l2: LinkedList<i32> = LinkedList::new();
        l1.push_back(1);
        l1.push_back(2);
        l1.push_back(3);
        l1.push_back(4);
        l1.push_back(5);

        l2.push_back(1);
        l2.push_back(1);
        l2.push_back(2);
        l2.push_back(3);
        l2.push_back(4);
        l2.push_back(3);
        l2.push_back(2);
        l2.push_back(1);
        l2.push_back(5);

        assert_eq!(l1, remove_duplicates(l2));
    }
}
