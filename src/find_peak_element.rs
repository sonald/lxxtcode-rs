pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        Self::by_log(nums)
    }

    // O(lgN)
    pub fn by_log(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len()-1);

        while l < r {
            let m = l + (r-l) / 2;
            if nums[m] > nums[m+1] {
                r = m;
            } else {
                l = m + 1;
            }
        }

        l as i32
    }

    // O(N)
    pub fn by_iter(nums: Vec<i32>) -> i32 {
        let mut up = true;
        for i in 0..nums.len() {
            if i == nums.len()-1 || nums[i] > nums[i+1] {
                if up {
                    return i as i32;
                }
                up = false;

            } else {
                up = true;
            }
        }

        -1

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method_test(f: impl Fn(Vec<i32>)->i32) {
        let v = vec![1,2,3,1];
        assert_eq!(f(v), 2);

        let v = vec![1,-4,3,1];
        let res = f(v);
        assert!([0,2].iter().any(|&x| x == res));

        let v = vec![i32::min_value(),2,1,2];
        let res = f(v);
        assert!([1,4].iter().any(|&x| x == res));

        let v = vec![1,2,1,3,5,6,4];
        let res = f(v);
        assert!([1,5].iter().any(|&x| x == res));
    }

    #[test]
    fn peak_test() {
        method_test(Solution::by_iter); 
        method_test(Solution::by_log); 
    }
}
