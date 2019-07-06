pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        (0..num_rows).for_each(|_| {
            let mut v = vec![1];
            if let Some(last) = res.last() {
                for j in 0..last.len() {
                    v.push(last[j..(j+2).min(last.len())].iter().sum());
                }
            }
            res.push(v);
        });

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_test() {
        {
            let v = vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ];
            assert_eq!(Solution::generate(5), v);
        }
        assert_eq!(Solution::generate(0), Vec::<Vec<i32>>::new());
    }
}
