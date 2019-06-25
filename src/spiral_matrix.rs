pub struct Solution;

impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        fn spiral(mat: &mut Vec<Vec<i32>>, res: &mut Vec<i32>, t: i32, r: i32, b: i32, l: i32) {
            if t > b || l > r {
                return;
            }

            mat[t as usize][l as usize..=r as usize].iter().for_each(|&i| res.push(i));
            (t+1..=b).for_each(|i| res.push(mat[i as usize][r as usize]));

            if t < b  {
                (l..r).rev().for_each(|j| res.push(mat[b as usize][j as usize]));
            }

            if l < r {
                (t+1..b).rev().for_each(|i| res.push(mat[i as usize][l as usize]));
            }
            spiral(mat, res, t+1, r-1, b-1, l+1);
        }

        let mut res = vec![];
        if matrix.len() > 0 { 
            let n = matrix.len() as i32;
            let m = matrix[0].len() as i32;
            res = Vec::with_capacity((n * m) as usize);
            spiral(&mut matrix, &mut res, 0, m-1, n-1, 0);
        }
        res
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spiral_test() {
        {
            let m = vec![vec![1]];
            let expect = vec![1];
            assert_eq!(Solution::spiral_order(m), expect);
        }
        {
            let m = vec![vec![7, 9, 6]];
            let expect = vec![7, 9, 6];
            assert_eq!(Solution::spiral_order(m), expect);
        }
        {
            let m = vec![vec![7], vec![9], vec![6]];
            let expect = vec![7, 9, 6];
            assert_eq!(Solution::spiral_order(m), expect);
        }
        {
            let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            let expect = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
            assert_eq!(Solution::spiral_order(m), expect);
        }
        {
            let m = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
            let expect = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
            assert_eq!(Solution::spiral_order(m), expect);
        }
        {
            let m = vec![
                vec![1, 2, 3], 
                vec![4, 5, 6],
                vec![7, 8, 9],
                vec![10, 11, 12]
            ];
            let expect = vec![1, 2, 3, 6, 9, 12, 11, 10, 7, 4, 5, 8];
            assert_eq!(Solution::spiral_order(m), expect);
        }
    }
}
