//! see [here](https://leetcode.com/problems/longest-palindromic-substring/)
//!

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        Solution::expand_from_center(s)
    }

    pub fn expand_from_center(s: String) -> String {
        fn solve(s: &[u8]) -> &[u8] {
            let mut res = (0, 0);
            let n = s.len();
            for c in 0..2*n-1 {
                let (mut i, mut j) = ((c) / 2, (c+1) / 2);
                while s[i] == s[j] {
                    if res.1 - res.0 < j - i {
                        res = (i, j);
                    }
                    if i == 0 || j == n-1 { break }
                    i -= 1;
                    j += 1;
                }
            }

            &s[res.0..=res.1]
        }

        if s.len() == 0 { return s };

        let t = solve(s.as_bytes());
        std::str::from_utf8(t).unwrap().to_string()
    }

    // with O(n) space
    pub fn dp2(s: String) -> String {
        fn solve(s: &[u8]) -> &[u8] {
            let len = s.len();

            let mut res = (0, 0);
            let mut dp = vec![vec![true; len]; 2];

            (0..len - 1).for_each(|i| {
                dp[1][i] = s[i] == s[i + 1];
                if dp[1][i] {
                    res = (i, i + 1);
                }
            });

            for sz in 3..=len {
                for i in 0..=len-sz {
                    let j = i + sz - 1;
                    dp[0][i] = dp[0][i + 1] && s[i] == s[j];

                    if dp[0][i] {
                        res = (i, j);
                    }
                }

                dp.swap(0, 1);
            }

            &s[res.0..=res.1]
        }

        if s.len() == 0 { return s };

        let t = solve(s.as_bytes());
        std::str::from_utf8(t).unwrap().to_string()

    }

    pub fn dp1(s: String) -> String {
        fn solve(s: &[u8]) -> &[u8] {
            let len = s.len();
            if len == 0 { return b""; };
            let mut res = (0, 0);
            let mut dp = vec![vec![false; len]; len];

            (0..len).for_each(|i| dp[i][i] = true);
            (0..len - 1).for_each(|i| {
                dp[i][i + 1] = s[i] == s[i + 1];
                if dp[i][i + 1] {
                    res = (i, i + 1);
                }
            });

            for sz in 3..=len {
                for i in 0..=len-sz {
                    let j = i + sz - 1;
                    dp[i][j] = dp[i + 1][j - 1] && s[i] == s[j];

                    if dp[i][j] {
                        res = (i, j);
                    }
                }
            }

            &s[res.0..=res.1]
        }

        let t = solve(s.as_bytes());
        std::str::from_utf8(t).unwrap().to_string()
    }

    // this is naive solution
    fn check_palindrome(s: &[u8]) -> Option<&[u8]> {
        let len = s.len();
        if len <= 1 {
            Some(s)
        } else if s[0] == s[len - 1] {
            match Solution::check_palindrome(&s[1..(len - 1)]) {
                Some(_) => Some(s),
                _ => None,
            }
        } else {
            Solution::check_palindrome(&s[0..(len - 1)]).and_then(|f| {
                match Solution::check_palindrome(&s[1..len]) {
                    Some(t) => {
                        if f.len() >= t.len() {
                            Some(f)
                        } else {
                            Some(t)
                        }
                    }
                    _ => Some(f),
                }
            })
        }
    }

    pub fn longest_palindrome_primitive(s: String) -> String {
        let b = s.as_bytes();
        let mut len = b.len();

        while len > 0 {
            for i in 0..=(b.len() - len) {
                let sub = &b[i..(i + len)];
                if Solution::is_palindrome(sub) {
                    return std::str::from_utf8(sub).unwrap().to_string();
                }
            }

            len -= 1;
        }

        "".to_string()
    }

    pub fn is_palindrome<T: AsRef<[u8]>>(t: T) -> bool {
        let s = t.as_ref();
        let (mut i, mut j) = (0, s.len() - 1);

        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    extern crate test;
    use super::*;
    use test::*;

    fn gen_random_string() -> String {
        use rand::random;
        (1..100).map(|_| char::from(random::<u8>() % 128)).collect()
    }

    #[test]
    fn test_primitive() {
        assert_eq!(
            Solution::longest_palindrome_primitive("babad".to_string()),
            "bab"
        );
        assert_eq!(
            Solution::longest_palindrome_primitive("cbbd".to_string()),
            "bb"
        );
        assert_eq!(Solution::longest_palindrome_primitive("ccc".to_string()), "ccc");
        assert_eq!(Solution::longest_palindrome_primitive("".to_string()), "");
    }

    #[test]
    fn test_expandaround() {
        assert_eq!(Solution::expand_from_center("babad".to_string()), "bab");
        assert_eq!(Solution::expand_from_center("cbbd".to_string()), "bb");
        assert_eq!(Solution::expand_from_center("caba".to_string()), "aba");
        assert_eq!(Solution::expand_from_center("ccc".to_string()), "ccc");
        assert_eq!(Solution::expand_from_center("".to_string()), "");

        for _ in 0..100{
            let s = gen_random_string();
            assert!(Solution::is_palindrome(Solution::expand_from_center(s)));
        }
    }

    #[test]
    fn test_dp2() {
        assert_eq!(Solution::dp2("babad".to_string()), "aba");
        assert_eq!(Solution::dp2("cbbd".to_string()), "bb");
        assert_eq!(Solution::dp2("caba".to_string()), "aba");
        assert_eq!(Solution::dp2("ccc".to_string()), "ccc");
        assert_eq!(Solution::dp2("".to_string()), "");

        for _ in 0..100{
            let s = gen_random_string();
            assert!(Solution::is_palindrome(Solution::dp2(s)));
        }
    }

    #[test]
    fn test_dp() {
        assert_eq!(Solution::dp1("babad".to_string()), "aba");
        assert_eq!(Solution::dp1("cbbd".to_string()), "bb");
        assert_eq!(Solution::dp1("caba".to_string()), "aba");
        assert_eq!(Solution::dp1("ccc".to_string()), "ccc");
        assert_eq!(Solution::dp1("".to_string()), "");

        for _ in 0..100{
            let s = gen_random_string();
            assert!(Solution::is_palindrome(Solution::dp1(s)));
        }
    }

    #[bench]
    fn bench_primitive(b: &mut Bencher) {
        b.iter(|| {
            let s = gen_random_string();
            Solution::longest_palindrome_primitive(s);
        })
    }

    #[bench]
    fn bench_dp1(b: &mut Bencher) {
        b.iter(|| {
            let s = gen_random_string();
            Solution::dp1(s);
        })
    }

    #[bench]
    fn bench_dp2(b: &mut Bencher) {
        b.iter(|| {
            let s = gen_random_string();
            Solution::dp2(s);
        })
    }

    #[bench]
    fn bench_expand(b: &mut Bencher) {
        b.iter(|| {
            let s = gen_random_string();
            Solution::expand_from_center(s);
        })
    }
}
