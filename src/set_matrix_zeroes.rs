pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        Self::by_marks(matrix)
    }

    fn by_constant_space(matrix: &mut Vec<Vec<i32>>) {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return;
        }

        let (nr, nc) = (matrix.len(), matrix[0].len());
        let mut empty_c = false;

        for r in 0..nr {
            if matrix[r][0] == 0 {
                empty_c = true;
            }

            for c in 1..nc {
                if matrix[r][c] == 0 {
                    matrix[r][0] = 0;
                    matrix[0][c] = 0;
                }
            }
        }

        // reverse order is important
        for r in (0..nr).rev() {
            for c in (1..nc).rev() {
                if matrix[r][0] == 0 || matrix[0][c] == 0 {
                    matrix[r][c] = 0;
                }
            }

            if empty_c {
                matrix[r][0] = 0;
            }
        }
    }

    fn by_marks(matrix: &mut Vec<Vec<i32>>) {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return;
        }

        let (nr, nc) = (matrix.len(), matrix[0].len());
        let mut rows = vec![false; nr];
        let mut cols = vec![false; nc];

        for r in 0..nr {
            for c in 0..nc {
                if matrix[r][c] == 0 {
                    rows[r] = true;
                    cols[c] = true;
                }
            }
        }

        for r in 0..nr {
            if rows[r] {
                let row = &mut matrix[r];
                for i in 0..row.len() {
                    row[i] = 0;
                }
            }
        }

        for c in 0..nc {
            if cols[c] {
                for i in 0..nr {
                    matrix[i][c] = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method_test(f: impl Fn(&mut Vec<Vec<i32>>)) {
        {
            let mut m = vec![
                vec![1,1,1],
                vec![1,0,1],
                vec![1,1,1]
            ];
            let o = vec![
                vec![1,0,1],
                vec![0,0,0],
                vec![1,0,1]
            ];
            f(&mut m);
            assert_eq!(m, o);
        }
        {
            let mut m = vec![
                vec![0,1,2,0],
                vec![3,4,5,2],
                vec![1,3,1,5]
            ];
            let o = vec![
                vec![0,0,0,0],
                vec![0,4,5,0],
                vec![0,3,1,0]
            ];
            f(&mut m);
            assert_eq!(m, o);
        }
    }

    #[test]
    fn dp_test() {
        method_test(Solution::by_marks);
        method_test(Solution::by_constant_space);
    }
}
