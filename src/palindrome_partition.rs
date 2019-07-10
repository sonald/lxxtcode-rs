pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        Self::by_dp(s)
    }

    pub fn by_backtrack(s: String) -> Vec<Vec<String>> {
        fn backtrack(s: &str, res: &mut Vec<Vec<String>>, sp: &mut Vec<String>) {
            if s.len() == 0 && sp.len() > 0 {
                res.push(sp.clone());
            }

            for i in 0..s.len() {
                if Solution::is_pal(s[0..=i].as_bytes()) {
                    let first = s[0..=i].to_owned();
                    sp.push(first);
                    backtrack(&s[i+1..], res, sp);
                    sp.pop();
                }
            }
        }

        let mut res = vec![];
        backtrack(&s, &mut res, &mut vec![]);
        res
    }

    pub fn by_dp(s: String) -> Vec<Vec<String>> {
        let s = &s;
        let n = s.len();
        let mut dp = vec![Vec::<Vec<String>>::new(); n+1];

        for i in (0..n).rev() {
            for j in i..n {
                if Self::is_pal(s[i..=j].as_bytes()) {
                    let first = s[i..=j].to_owned();
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
    extern crate test;
    use test::Bencher;

    #[test]
    fn partition_test() {
        method_test(Solution::by_backtrack);
        method_test(Solution::by_dp);
    }

    fn method_test(f: impl Fn(String) -> Vec<Vec<String>>) {
        {
            let expect = Vec::<Vec<String>>::new();
            let res = f("".to_string());
            assert_eq!(res, expect);
        }
        {
            let expect = vec![vec!["a"]];
            let res = f("a".to_string());
            assert_eq!(res, expect);
        }
        {
            let mut expect = vec![vec!["aa", "b"], vec!["a", "a", "b"]];
            expect.sort();
            let mut res = f("aab".to_string());
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
            let mut res = f("aaaba".to_string());
            res.sort();
            assert_eq!(res, expect);
        }
    }

    #[bench]
    fn bench_bt(b: &mut test::Bencher) {
        b.iter(|| {
        {
            Solution::by_backtrack("aaaaaaaaa".to_string());
        }
            
        })
    }

    #[bench]
    fn bench_dp(b: &mut test::Bencher) {
        b.iter(|| {
        {
            Solution::by_dp("aaaaaaaaa".to_string());
        }
            
        })
    }
}
