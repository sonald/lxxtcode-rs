pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::by_power(nums)
    }

    fn by_backtrack(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        fn backtrack(res: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, sp: &mut Vec<i32>) {
            if let Some(i) = nums.pop() {
                backtrack(res, nums, sp);

                sp.push(i);
                backtrack(res, nums, sp);
                nums.push(sp.pop().unwrap());

            } else {
                res.push(sp.clone());
            }
        }

        backtrack(&mut res, &mut nums, &mut vec![]);

        res
    }

    fn by_power(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let n = nums.len();
        let m = 2u64.pow(n as u32);

        for i in 0..m {
            let mut sp = vec![];
            (0..n).filter(|j| (i & (1<<j) as u64) != 0).for_each(|j| sp.push(nums[j]));
            res.push(sp);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method_test(f: impl Fn(Vec<i32>) -> Vec<Vec<i32>>) {
        let v = vec![1,2,3];
        let mut expect = vec!{
            vec![3],
            vec![1],
            vec![2],
            vec![1,2,3],
            vec![1,3],
            vec![2,3],
            vec![1,2],
            vec![]
        };
        
        let mut res = f(v);
        res.iter_mut().for_each(|v| v.sort());
        res.sort();
        expect.sort();
        assert_eq!(res, expect);
    }


    #[test]
    fn power_test() {
        method_test(Solution::by_power);
        method_test(Solution::by_backtrack);
    }
}
