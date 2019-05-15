//! see [here]()
//!

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        //Solution::base(nums.as_mut_slice(), k)
        Solution::with_heap(nums.as_mut_slice(), k)
    }

    fn with_heap(nums: &mut [i32], k: i32) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        
        let mut h = BinaryHeap::new();
        let k = k as usize;
        for i in 0..nums.len() {
            h.push(Reverse(nums[i]));
            if i >= k {
                h.pop();
            }
        }
        
        h.peek().unwrap().0
    }

    fn base(nums: &mut [i32], k: i32) -> i32 {
        nums.sort();
        nums[nums.len() - k as usize]
    }
}

#[allow(dead_code)]
struct Solution2;

#[allow(dead_code)]
impl Solution2 {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = (k - 1) as usize;
        let mut nums = nums;
        quick_select(&mut nums[..], k)
    }
}

fn partition(nums: &mut [i32], pivot_index: usize) -> usize {
    let pivot = nums[pivot_index];
    let mut l = 0;
    let mut r = nums.len() - 1;
    loop {
        while l < r && nums[l] > pivot { l += 1 }
        while l < r && nums[r] <= pivot { r -= 1 }
        if l < r { nums.swap(l, r) } else { break }
    }
    l
}

fn median_of_three(nums: &[i32]) -> usize {
    let start = 0;
    let end = nums.len() - 1;    
    let mid = (start + end) / 2;
    if nums[start] >= nums[mid] && nums[mid] >= nums[end] {
        mid
    } else if nums[start] <= nums[mid] && nums[end] <= nums[start] {
        start
    } else {
        end
    }    
}

fn quick_select(nums: &mut [i32], k: usize) -> i32 {
    if nums.len() < 2 {
        return nums[k];
    }
    let i = median_of_three(nums);
    let mid = partition(nums, i);
    if mid > 0 {
        if mid > k {
            quick_select(&mut nums[0..mid], k)
        } else {
            quick_select(&mut nums[mid..], k - mid)
        }
    } else {
        // nums[i] is the largest
        if k == 0 {
            nums[i]
        } else {
            nums.swap(0, i);
            quick_select(&mut nums[1..], k - 1)
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    extern crate test;
    use super::*;
    use test::*;

    fn gen_random_string() -> String {
        use rand::random;
        (1..100).map(|_| char::from(random::<u8>() % 128)).collect()
    }


    #[test]
    fn test_primitive() {
        {
            let v = vec![2,3,1,4,5,6];
            let k = 6;
            assert_eq!(Solution::find_kth_largest(v, k), 1);
        }
        {
            let v = vec![2,3,1,2,4,5,5,6];
            let k = 1;
            assert_eq!(Solution::find_kth_largest(v, k), 6);
        }
        {
            let v = vec![3,2,3,1,2,4,5,5,6];
            let k = 4;
            assert_eq!(Solution::find_kth_largest(v, k), 4);
        }
        {
            let v = vec![3,2,1,5,6,4];
            let k = 2;
            assert_eq!(Solution::find_kth_largest(v, k), 5);
        }
    }

    #[bench]
    fn bench_expand(b: &mut Bencher) {
        b.iter(|| {
        })
    }
}


