pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut acc = max;
        for &x in &nums[1..] {
            acc = x.max(acc+x);
            if acc > max {
                max = acc;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method_test(f: impl Fn(Vec<i32>)->i32) {
        {
            let v = vec![1];
            assert_eq!(f(v), 1);
        }
        {
            let v = vec![-2];
            assert_eq!(f(v), -2);
        }
        {
            let v = vec![-2,1,-3,4,-1,2,1,-5,4];
            assert_eq!(f(v), 6); // Explanation: [4,-1,2,1]
        }
        {
            let v = vec![8,-19,5,-4,20];
            assert_eq!(f(v), 21);
        }
        
    }

    #[test]
    fn subarray_test() {
        method_test(Solution::max_sub_array);
    }
}

