//! see [here]()
//!

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        Solution::by_map(&nums)
        //let mut nums = nums;
        //Solution::by_sort(&mut nums)
    }

    fn by_sort(nums: &mut [i32]) -> bool {
        if nums.len() == 0 { return false; }
        nums.sort();

        for i in 0..nums.len()-1 {
            if nums[i] == nums[i+1] {
                return true
            }
        }

        false
    }

    // O(n) + O(n)
    fn by_map(nums: &[i32]) -> bool {
        use std::collections::HashSet;
        if nums.len() == 0 { return false; }

        let mut h = HashSet::new();
        for d in nums {
            match h.get(&d) {
                None => { h.insert(d); },
                _ => return true
            }
        }

        false
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    extern crate test;
    use super::*;
    use test::*;

    fn gen_random_arrays() -> Vec<i32> {
        use rand::random;
        (1..1000).map(|_| random()).collect()
    }


    #[test]
    fn test_map() {
        assert_eq!(Solution::contains_duplicate(vec![]), false);
        {
            let v = vec![2,3,1,4,2,6];
            assert_eq!(Solution::contains_duplicate(v), true);
        }
        {
            let v = vec![2,3,1,4,5,6];
            assert_eq!(Solution::contains_duplicate(v), false);
        }
    }

    #[test]
    fn test_sort() {
        assert_eq!(Solution::by_sort(&mut vec![]), false);
        {
            let mut v = vec![2,3,1,4,2,6];
            assert_eq!(Solution::by_sort(&mut v), true);
        }
        {
            let mut v = vec![2,3,1,4,5,6];
            assert_eq!(Solution::by_sort(&mut v), false);
        }
    }

    #[bench]
    fn bench_map(b: &mut Bencher) {
        b.iter(|| {
            let v = gen_random_arrays();
            Solution::by_map(&v);
        })
    }

    #[bench]
    fn bench_sort(b: &mut Bencher) {
        b.iter(|| {
            let mut v = gen_random_arrays();
            Solution::by_sort(&mut v);
        })
    }
}



