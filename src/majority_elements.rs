pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut count = 1;
        for &x in &nums[1..] {
            count += match res != x {
                true => -1,
                _ => 1
            };

            if count < 0 {
                res = x;
                count = 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn major_test() {
        let v = vec![3];
        assert_eq!(Solution::majority_element(v), 3);

        let v = vec![6,5,5];
        assert_eq!(Solution::majority_element(v), 5);

        let v = vec![3,1,2,3,1,2,3];
        assert_eq!(Solution::majority_element(v), 3);

        let v = vec![3,2,3];
        assert_eq!(Solution::majority_element(v), 3);

        let v = vec! [2,2,1,1,1,2,2];
        assert_eq!(Solution::majority_element(v), 2);
    }
}
