pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        Self::by_constant_dp(s)
    }

    pub fn by_constant_dp(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![0;2];
        dp[0] = 1;
        dp[1] = if s[n-1..n].parse::<i32>().unwrap() == 0 {
            0
        } else {
            1
        };
        for i in 2..=n {
            let mut d;
            let k = s[n-i..n-i+2].parse::<i32>().unwrap();
            if k == 10 || k == 20 {
                d = dp[0];
            } else if k < 10 {
                d = 0;
            } else {
                d = dp[1];
                if k <= 26 {
                    d += dp[0];
                }
            }

            dp[0] = dp[1];
            dp[1] = d;
        }
        
        dp[1]

    }

    pub fn by_dp(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![0; n+1];
        dp[0] = 1;
        dp[1] = if s[n-1..n].parse::<i32>().unwrap() == 0 {
            0
        } else {
            1
        };
        for i in 2..=n {
            let k = s[n-i..n-i+2].parse::<i32>().unwrap();
            if k == 10 || k == 20 {
                dp[i] = dp[i-2];
            } else if k < 10 {
                dp[i] = 0;
            } else {
                dp[i] = dp[i-1];
                if k <= 26 {
                    dp[i] += dp[i-2];
                }
            }
        }
        
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method_test(f: impl Fn(String)->i32) {
        assert_eq!(f("12".to_string()), 2);
        assert_eq!(f("226".to_string()), 3);
        assert_eq!(f("1226".to_string()), 5);
        assert_eq!(f("1026".to_string()), 2);
        assert_eq!(f("1206".to_string()), 1);
        assert_eq!(f("1220".to_string()), 2);
    }

    #[test]
    fn ways_test() {
        method_test(Solution::by_dp);
        method_test(Solution::by_constant_dp);
    }
}
