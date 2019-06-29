pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        Self::by_oneline_dp(m, n)
    }

    pub fn by_full_dp(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;
        let mut dp = vec![vec![0; m+1]; n+1];
        dp[1][0] = 1;
        for r in 1..=n {
            for c in 1..=m {
                dp[r][c] = dp[r-1][c] + dp[r][c-1];
            }
        }

        dp[n][m]
    }

    // use one line 
    pub fn by_oneline_dp(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;
        let mut dp = vec![0; m+1];

        dp[1] = 1;
        (0..n).for_each(|_| {
            for c in 1..=m {
                let new = dp[c-1] + dp[c];
                dp[c] = new;
            }
        });

        dp[m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method_test(f: impl Fn(i32, i32)->i32) {
        assert_eq!(f(3, 2), 3);
        assert_eq!(f(7, 3), 28);
    }

    #[test]
    fn dp_test() {
        method_test(Solution::by_full_dp);
        method_test(Solution::by_oneline_dp);
    }
}

