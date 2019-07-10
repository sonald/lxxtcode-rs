pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::by_iter(&nums, 0, nums.len() as i32 - 1, target)
        //        Self::bst(&nums, 0, nums.len() as i32 - 1, target)
    }

    fn bst(nums: &Vec<i32>, mut l: i32, mut r: i32, target: i32) -> i32 {
        use std::cmp::Ordering::*;

        while l <= r {
            let m = l + (r - l) / 2;
            match nums[m as usize].cmp(&target) {
                Less => {
                    if nums[m as usize] >= nums[0] {
                        l = m + 1;
                    } else {
                        let res = Self::bst(nums, m + 1, r, target);
                        if res >= 0 {
                            return res;
                        }
                        r = m - 1;
                    }
                }
                Greater => {
                    if nums[m as usize] >= nums[0] {
                        let res = Self::bst(nums, l, m - 1, target);
                        if res >= 0 {
                            return res;
                        }
                        l = m + 1;
                    } else {
                        r = m - 1;
                    }
                }
                Equal => return m as i32,
            }
        }

        -1
    }

    fn by_iter(nums: &Vec<i32>, mut l: i32, mut r: i32, target: i32) -> i32 {
        while l <= r {
            let m = l + (r - l) / 2;
            if nums[m as usize] == target {
                return m;
            }
            match nums[m as usize] >= nums[0] {
                true => {
                    if nums[m as usize] > target && nums[0] <= target {
                        r = m - 1;
                    } else {
                        l = m + 1;
                    }
                }
                false => {
                    if nums[m as usize] < target && target <= *nums.last().unwrap() {
                        l = m + 1;
                    } else {
                        r = m - 1;
                    }
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn test_method(f: impl Fn(Vec<i32>, i32) -> i32) {
        assert_eq!(f(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(f(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        for i in 0..2 {
            assert_eq!(f(vec![4, 5, 6, 7, 0, 1, 2], i), i + 4);
        }
        for i in 4..=7 {
            assert_eq!(f(vec![4, 5, 6, 7, 0, 1, 2], i), i - 4);
        }
        assert_eq!(f(vec![5, 1, 3], 3), 2);
    }

    #[test]
    fn test_search() {
        test_method(Solution::search)
    }
}
