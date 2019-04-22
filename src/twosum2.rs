use std::vec::Vec;
use std::collections::HashMap;

struct Solution;
struct Solution2;

impl Solution {
    //O(1) space + O(nlgn) time
    #[allow(unused)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        for (f, cf) in nums.iter().enumerate() {
            let (mut l, mut r) = (0, n-1);

            while l <= r {
                let m = (l+r)/2;
                
                match cf + nums[m] {
                    res if res < target => l = m+1,
                    res if res > target => r = m-1,
                    _ => {
                        if f == m { break }
                        match f > m {
                            true => return vec![m as i32, f as i32],
                            _ => return vec![f as i32, m as i32],
                        }
                    }
                }
            }
        }

        unreachable!()
    }
}

impl Solution2 {

    //O(1) space + O(n) time
    #[allow(unused)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, nums.len()-1);

        while l <= r {
            match nums[l] + nums[r] {
                res if res == target => return vec![l as i32, r as i32],
                res if res > target => r -= 1,
                _ => l += 1,
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sol() {
        {
            let v = vec![2, 7, 11, 15];
            assert_eq!(Solution::two_sum(v, 9), vec![0,1]);
        }
        {
            let v = vec![-1, 0, 1];
            assert_eq!(Solution::two_sum(v, 0), vec![0,2]);
        }
        {
            let v = vec![0, 0, 3, 4];
            assert_eq!(Solution::two_sum(v, 0), vec![0,1]);
        }
    }

    #[test]
    pub fn test_sol2() {
        {
            let v = vec![2, 7, 11, 15];
            assert_eq!(Solution2::two_sum(v, 9), vec![0,1]);
        }
        {
            let v = vec![-1, 0, 1];
            assert_eq!(Solution2::two_sum(v, 0), vec![0,2]);
        }
        {
            let v = vec![0, 0, 3, 4];
            assert_eq!(Solution2::two_sum(v, 0), vec![0,1]);
        }
    }
}

