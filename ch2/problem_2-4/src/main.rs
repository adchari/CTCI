use std::collections::LinkedList;

#[allow(dead_code)]
fn partition<T: PartialOrd + Copy>(list: LinkedList<T>, partition: T) -> LinkedList<T> {
    let mut lt: LinkedList<T> = LinkedList::new();
    let mut geq: LinkedList<T> = LinkedList::new();
    for it in list.iter() {
        if *it < partition {
            lt.push_back(*it);
        } else {
            geq.push_back(*it);
        }
    }
    lt.append(&mut geq);
    return lt;
}

fn main() {
    // Unimplemented
}
