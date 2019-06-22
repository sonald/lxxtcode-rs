pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::by_recurse(nums)
    }

    pub fn by_recurse(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let mut res = vec![];
        let mut sp = vec![];

        fn recurse(sp: &mut Vec<i32>, free: &HashSet<i32>, res: &mut Vec<Vec<i32>>) {
            if free.len() == 0 {
                res.push(sp.clone());
                return;
            }

            let mut free2 = free.clone();
            for u in free.iter() {
                free2.remove(u);
                sp.push(*u);
                recurse(sp, &free2, res);
                sp.pop();
                free2.insert(*u);
            }
        }

        let free = nums.into_iter().collect::<HashSet<_>>();
        recurse(&mut sp, &free, &mut res);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurse() {
        test_method(Solution::by_recurse);
    }

    fn test_method(f: impl Fn(Vec<i32>) -> Vec<Vec<i32>>) {
        {
            let mut res = f(vec![1, 2, 3]);
            res.sort();
            let mut expect = vec![
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1],
            ];
            expect.sort();

            assert_eq!(res, expect);
        }
        {
            let mut res = f(vec![5, 4, 6, 2]);
            res.sort();
            let mut expect = vec![
                [5, 4, 6, 2],
                [5, 4, 2, 6],
                [5, 6, 4, 2],
                [5, 6, 2, 4],
                [5, 2, 4, 6],
                [5, 2, 6, 4],
                [4, 5, 6, 2],
                [4, 5, 2, 6],
                [4, 6, 5, 2],
                [4, 6, 2, 5],
                [4, 2, 5, 6],
                [4, 2, 6, 5],
                [6, 5, 4, 2],
                [6, 5, 2, 4],
                [6, 4, 5, 2],
                [6, 4, 2, 5],
                [6, 2, 5, 4],
                [6, 2, 4, 5],
                [2, 5, 4, 6],
                [2, 5, 6, 4],
                [2, 4, 5, 6],
                [2, 4, 6, 5],
                [2, 6, 5, 4],
                [2, 6, 4, 5],
            ];
            expect.sort();

            assert_eq!(res, expect);
        }
    }
}
