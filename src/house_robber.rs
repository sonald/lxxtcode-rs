pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        Self::by_dp_constant(nums)
    }

    pub fn by_dp_constant(nums: Vec<i32>) -> i32 {
        let (mut cur1, mut cur2) = (0, 0);

        for i in 0..nums.len() {
            let new = cur1.max(cur2+nums[i]);
            cur2 = cur1;
            cur1 = new;
        }

        cur1.max(cur2)
    }

    pub fn by_dp(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0; }
        let mut dp = vec![0; nums.len()+2];

        for i in 2..=nums.len()+1 {
            dp[i] = dp[i-1].max(nums[i-2] + dp[i-2]);
        }

        dp[nums.len()+1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rob_test() {
        assert_eq!(Solution::rob(vec![]), 0);
        let v = vec![1,2];
        assert_eq!(Solution::rob(v), 2);
        let v = vec![1,2,3,1];
        assert_eq!(Solution::rob(v), 4);
        let v = vec![2,7,9,3,1];
        assert_eq!(Solution::rob(v), 12);
    }
}
