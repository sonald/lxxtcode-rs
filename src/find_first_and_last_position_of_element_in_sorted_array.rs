pub struct Solution;

impl Solution {

    //NOTE: do not use Vec's builtin binary search
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering::*;

        let (mut l, mut r) = (0, nums.len() as i32 - 1);
        let mut res = vec![-1, -1];
        let mut i = -1;

        while l <= r {
            let m = l + (r - l) / 2;
            match nums[m as usize].cmp(&target) {
                Less => l = m + 1,
                Greater => r = m - 1,
                Equal => {
                    i = m;
                    break;
                }
            }
        }

        if i == -1 {
            return res;
        }

        // find lower bound and upper bound
        res[0] = if nums[0] == target {
            0
        } else {
            l = 0;
            r = i;
            while l < r {
                let m = l + (r - l) / 2;
                match nums[m as usize].cmp(&target) {
                    Less => l = m+1,
                    _ =>  r = m-1
                }
            }
            
            if nums[r as  usize] == target {
                r
            } else {
                r+1
            }
        };

        res[1] = if *nums.last().unwrap() == target {
            nums.len() as i32 - 1
        } else {
            l = i;
            r = nums.len() as i32 - 1;
            while l < r {
                let m = l + (r - l) / 2;
                match nums[m as usize].cmp(&target) {
                    Greater => r = m-1,
                    _ =>  l = m+1
                }
            }
            if nums[l as  usize] == target {
                l
            } else {
                l-1
            }
        };


        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn test_method(f: impl Fn(Vec<i32>, i32) -> Vec<i32>) {
        assert_eq!(f(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(f(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
        assert_eq!(f(vec![7, 7, 7, 7, 7, 7], 7), vec![0, 5]);
        assert_eq!(f(vec![7, 7, 7, 7, 7, 7, 7], 7), vec![0, 6]);
        assert_eq!(f(vec![5, 7, 7, 8, 8, 8, 10], 8), vec![3, 5]);
        assert_eq!(f(vec![5], 5), vec![0, 0]);
        assert_eq!(f(vec![], 5), vec![-1, -1]);
    }

    #[test]
    fn test_search_range() {
        test_method(Solution::search_range)
    }
}
