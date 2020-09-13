use std::collections::LinkedList;
fn add(n1: LinkedList<i32>, n2: LinkedList<i32>) -> LinkedList<i32> {
    let mut carry: i32 = 0;
    let mut n1_iter = n1.iter();
    let mut n2_iter = n2.iter();
    let mut ret = LinkedList::<i32>::new();
    loop {
        match (n1_iter.next(), n2_iter.next()) {
            (Some(x), Some(y)) => {
                ret.push_back((x + y + carry) % 10);
                carry = (x + y + carry) / 10;
            }
            (Some(x), None) => {
                ret.push_back((x + carry) % 10);
                carry = (x + carry) / 10;
            }
            (None, Some(y)) => {
                ret.push_back((y + carry) % 10);
                carry = (y + carry) / 10;
            }
            (None, None) => {
                while carry != 0 {
                    ret.push_back(carry % 10);
                    carry = carry / 10;
                }
                break;
            }
        }
    }
    return ret;
}

fn main() {
    // Unimplemented
}

#[cfg(test)]
mod test {
    use crate::add;
    use std::collections::LinkedList;
    #[test]
    fn example() {
        let mut l1: LinkedList<i32> = LinkedList::new();
        l1.push_back(7);
        l1.push_back(1);
        l1.push_back(6);

        let mut l2: LinkedList<i32> = LinkedList::new();
        l2.push_back(5);
        l2.push_back(9);
        l2.push_back(2);
        l2.push_back(1);

        let mut l3: LinkedList<i32> = LinkedList::new();
        l3.push_back(2);
        l3.push_back(1);
        l3.push_back(9);
        l3.push_back(1);
        assert_eq!(l3, add(l1, l2));
    }
}
