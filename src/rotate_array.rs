pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        Self::by_swaps(nums, k)
    }

    pub fn by_shift(nums: &mut Vec<i32>, k: i32) {
        fn shift(nums: &mut Vec<i32>) {
            if let Some(&d) = nums.last() {
                for i in (0..nums.len()-1).rev() {
                    nums[i+1] = nums[i];
                }
                nums[0] = d;
            }
        }

        let k = (k as usize) % nums.len();
        if k == 0 { return; }
        (0..k).for_each(|_| shift(nums));
    }

    pub fn by_swaps(nums: &mut Vec<i32>, k: i32) {
        fn swap(nums: &mut [i32]) {
            for i in 0..nums.len()/2 {
                nums.swap(i, nums.len()-1-i);
            }
        }

        let k = (k as usize) % nums.len();
        if k == 0 { return; }
        
        swap(&mut nums[..]);
        swap(&mut nums[0..k]);
        swap(&mut nums[k..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_test() {
        method_test(Solution::by_shift);
        method_test(Solution::by_swaps);
    }

    fn method_test(f: impl Fn(&mut Vec<i32>, i32)) {
        {
            let mut v = vec![1,2,3,4,5,6,7];
            f(&mut v, 0);
            assert_eq!(v, vec![1,2,3,4,5,6,7]);
        }
        {
            let mut v = vec![1,2,3,4,5,6,7];
            f(&mut v, 14);
            assert_eq!(v, vec![1,2,3,4,5,6,7]);
        }
        {
            let mut v = vec![1,2,3,4,5,6,7];
            f(&mut v, 7);
            assert_eq!(v, vec![1,2,3,4,5,6,7]);
        }
        {
            let mut v = vec![1,2,3,4,5,6,7];
            f(&mut v, 1);
            assert_eq!(v, vec![7,1,2,3,4,5,6]);
        }
        {
            let mut v = vec![1,2,3,4,5,6,7];
            f(&mut v, 8);
            assert_eq!(v, vec![7,1,2,3,4,5,6]);
        }
        {
            let mut v = vec![1,2,3,4,5,6,7];
            f(&mut v, 2);
            assert_eq!(v, vec![6,7,1,2,3,4,5]);
        }
        {
            let mut v = vec![1,2,3,4,5,6,7];
            f(&mut v, 3);
            assert_eq!(v, vec![5,6,7,1,2,3,4]);
        }
        {
            let mut v = vec![1,2,3,4,5,6,7];
            f(&mut v, 4);
            assert_eq!(v, vec![4, 5,6,7,1,2,3]);
        }
        
    }
}
