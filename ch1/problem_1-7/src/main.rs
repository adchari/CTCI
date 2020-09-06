use std::vec::Vec;

fn rotate(inp: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let mut m: Vec<Vec<isize>> = Vec::new();
    m.resize(inp[0].len(), Vec::new());
    for row in m.iter_mut() {
        row.resize(inp.len(), -1);
    }

    for (i, e) in inp.iter().enumerate() {
        for (j, f) in e.iter().enumerate() {
            m[inp[0].len() - j - 1][i] = *f;
        }
    }
    return m;
}

fn main() {
    // not implemented
}

#[cfg(test)]
mod test {
    use crate::rotate;
    #[test]
    fn example() {
        let m1 = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let m1_rot = vec![
            vec![4, 8, 12],
            vec![3, 7, 11],
            vec![2, 6, 10],
            vec![1, 5, 9],
        ];
        assert_eq!(m1_rot, rotate(m1));
    }
}
