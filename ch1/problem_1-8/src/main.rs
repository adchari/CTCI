use std::vec::Vec;

fn zero_out(inp: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let mut m = inp.clone();

    for (i, e) in inp.iter().enumerate() {
        for (j, f) in e.iter().enumerate() {
            if *f == 0 {
                for x in 0..inp.len() {
                    m[x][j] = 0;
                }
                for y in 0..inp[i].len() {
                    m[i][y] = 0;
                }
            }
        }
    }
    return m;
}

fn main() {
    // not implemented
}

#[cfg(test)]
mod test {
    use crate::zero_out;
    #[test]
    fn example() {
        let m1 = vec![vec![1, 2, 0, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let m1_zero = vec![vec![0, 0, 0, 0], vec![5, 6, 0, 8], vec![9, 10, 0, 12]];
        assert_eq!(m1_zero, zero_out(m1));
    }
}
