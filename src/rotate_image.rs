pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..(n+1)/2 {
            for j in i..(n-i-1) {
                let t = matrix[i][j];
                matrix[i][j] = matrix[n-j-1][i];
                matrix[n-j-1][i] = matrix[n-i-1][n-j-1];
                matrix[n-i-1][n-j-1] = matrix[j][n-i-1];
                matrix[j][n-i-1] = t;

            }
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_test() {
        {
            let mut res = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

            Solution::rotate(&mut res);

            let expect = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
            assert_eq!(res, expect);
        }

        {
            let mut res = vec![
                vec![5, 1, 9, 11],
                vec![2, 4, 8, 10],
                vec![13, 3, 6, 7],
                vec![15, 14, 12, 16],
            ];

            Solution::rotate(&mut res);

            let expect = vec![
                [15, 13, 2, 5],
                [14, 3, 4, 1],
                [12, 6, 8, 9],
                [16, 7, 10, 11],
            ];
            assert_eq!(res, expect);
        }
    }
} /* tests */
