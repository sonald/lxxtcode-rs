pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        Self::by_dp(s, word_dict) 
    }

    fn by_dp(s: String, word_dict: Vec<String>) -> bool {
        use std::collections::HashSet;

        let s = &s;
        let h = word_dict.iter().map(|s| s.as_str()).collect::<HashSet<_>>();
        let mut dp = vec![false; s.len()+1];
        dp[s.len()] = true;

        for i in (0..s.len()).rev() {
            for j in i..s.len() {
                if h.contains(&s[i..=j]) && dp[j+1] {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn break_test() {
        method_test(Solution::by_dp);        
    }

    fn method_test(f: impl Fn(String, Vec<String>) -> bool) {
        {
            let s = "leetcode".to_string();
            let word_dict = vec!["leet".to_string(), "code".to_string()];
            assert_eq!(f(s, word_dict), true);
        }
        {
            let s = "applepenapple".to_string();
            let word_dict = vec!["apple".to_string(), "pen".to_string()];
            assert_eq!(f(s, word_dict), true);
        }
        {
            let s = "appleponapple".to_string();
            let word_dict = vec!["apple".to_string(), "pen".to_string()];
            assert_eq!(f(s, word_dict), false);
        }
    }
}
