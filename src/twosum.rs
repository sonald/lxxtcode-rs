use std::vec::Vec;
use std::collections::HashMap;

struct Solution;
struct Solution2;

impl Solution {
    #[allow(unused)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut c = nums.clone();

        c.sort_unstable();

        let n = c.len();

        for (f, cf) in c.iter().enumerate() {
            let (mut l, mut r) = (0, n-1);

            while l <= r {
                let m = (l+r)/2;
                
                match cf + c[m] {
                    res if res < target => l = m+1,
                    res if res > target => r = m-1,
                    _ => {
                        if f == m { break }
                        let x = nums.iter().position(|&v| v == *cf).unwrap() as i32;
                        let y = nums.iter().rposition(|&v| v == c[m]).unwrap() as i32;
                        match x > y {
                            true => return vec![y, x],
                            _ => return vec![x, y],
                        }
                    }
                }
            }
        }

        vec![0, 0]
    }
}

impl Solution2 {
    #[allow(unused)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h = HashMap::with_capacity(nums.len());

        for (i, v) in nums.iter().enumerate() {
            let u = target - v;
            match h.get(&u) {
                Some(&idx) => return vec![idx, i as i32],
                _ =>  { h.insert(v, i as i32); },
            }
        }

        vec![0, 0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_twosum() {
        {
            let v = vec![15, 2, 7, 11];
            //println!("{:?}", Solution::two_sum(v, 9));
            assert_eq!(Solution::two_sum(v, 9), vec![1,2]);
        }
        {
            let v = vec![1, 0, -1];
            //println!("{:?}", Solution::two_sum(v, 0));
            assert_eq!(Solution::two_sum(v, 0), vec![0,2]);
        }
        {
            let v = vec![0, 4, 3, 0];
            //println!("{:?}", Solution::two_sum(v, 0));
            assert_eq!(Solution::two_sum(v, 0), vec![0,3]);
        }
    }

    #[test]
    pub fn test_twosum2() {
        {
            let v = vec![15, 2, 7, 11];
            //println!("{:?}", Solution::two_sum(v, 9));
            assert_eq!(Solution2::two_sum(v, 9), vec![1,2]);
        }
        {
            let v = vec![1, 0, -1];
            //println!("{:?}", Solution::two_sum(v, 0));
            assert_eq!(Solution2::two_sum(v, 0), vec![0,2]);
        }
        {
            let v = vec![0, 4, 3, 0];
            //println!("{:?}", Solution::two_sum(v, 0));
            assert_eq!(Solution2::two_sum(v, 0), vec![0,3]);
        }
    }
}
