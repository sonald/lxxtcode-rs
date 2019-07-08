pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut dp = vec![Vec::<Vec<String>>::new(); n+1];

        for i in (0..n).rev() {
            for j in i..n {
                if Self::is_pal(&bytes[i..=j]) {
                    let first = String::from_utf8(bytes[i..=j].to_vec()).unwrap();
                    if j == n-1 {
                        let v = vec![first.clone()];
                        dp[i].push(v);
                    } else {
                        let mut vs = vec![];
                        for v2 in &dp[j+1] {
                            let mut v = vec![first.clone()];
                            v.append(&mut v2.clone());
                            vs.push(v);
                        }

                        dp[i].append(&mut vs);
                    }
                }
            }
        }

        dp[0].clone()
    }

    fn is_pal(s: &[u8]) -> bool {
        for i in 0..s.len()/2 {
            if s[i] != s[s.len()-1-i] {
                return false
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_test() {
        {
            let expect = Vec::<Vec<String>>::new();
            let res = Solution::partition("".to_string());
            assert_eq!(res, expect);
        }
        {
            let expect = vec![vec!["a"]];
            let res = Solution::partition("a".to_string());
            assert_eq!(res, expect);
        }
        {
            let mut expect = vec![vec!["aa", "b"], vec!["a", "a", "b"]];
            expect.sort();
            let mut res = Solution::partition("aab".to_string());
            res.sort();

            assert_eq!(res, expect);
        }
        {
            let mut expect = vec![
                vec!["a", "a", "a", "b", "a"],
                vec!["a", "a", "aba"],
                vec!["a", "aa", "b", "a"],
                vec!["aa", "a", "b", "a"],
                vec!["aa", "aba"],
                vec!["aaa", "b", "a"],
            ];
            expect.sort();
            let mut res = Solution::partition("aaaba".to_string());
            res.sort();
            assert_eq!(res, expect);
        }
    }
}
