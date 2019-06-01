use std::vec::Vec;

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        //Solution::by_iter(digits)
        Solution::by_generative(digits)
    }

    pub fn by_fold(digits: String) -> Vec<String> {
        if digits.len() == 0 { return vec![]; }

        let m = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut s = String::new();

        digits.chars().fold(vec!["".to_string()], |mut res, ch| {
            let mut new = vec![];
            let idx = ch.to_digit(10).unwrap() as usize;
            while let Some(s) = res.pop() {
                for ch in m[idx].chars() {
                    let mut t = s.clone();
                    t.push(ch);
                    new.push(t);
                }
            }
            new
        })
    }

    pub fn by_generative(digits: String) -> Vec<String> {
        if digits.len() == 0 { return vec![]; }

        let m = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut res = vec![];
        let mut s = String::new();

        fn inner(chars: &[u8], idx: usize, s: &mut String, res: &mut Vec<String>, m: &[&str]) {
            if idx < chars.len() {
                let i = chars[idx] as usize - 48;
                for ch in m[i].chars() {
                    s.push(ch);
                    inner(chars, idx+1, s, res, m);
                    s.pop();
                }
            } else {
                res.push(s.clone());
            }
        }

        inner(&digits.as_bytes(), 0, &mut s, &mut res, &m);

        res
    }

    pub fn by_iter(digits: String) -> Vec<String> {
        if digits.len() == 0 { return vec![]; }

        let m = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut res = vec!["".to_string()];

        for ch in digits.chars() {
            let idx = ch.to_digit(10).unwrap() as usize;
            let mut res2 = vec![];
            while let Some(s) = res.pop() {
                for ch in m[idx].chars() {
                    let mut t = s.clone();
                    t.push(ch);
                    res2.push(t);
                }
            }

            res = res2;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    pub fn test_method<F>(f: F) where F: Fn(String) -> Vec<String> {
        {
            let res = f("23".to_string()).into_iter().collect::<HashSet<_>>();
            let expect = ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].iter().map(|s| s.to_string()).collect::<HashSet<_>>();
            assert_eq!(res, expect);
        }
        {
            let res = f("7".to_string()).into_iter().collect::<HashSet<_>>();
            let expect = ["p", "q", "s", "r"].iter().map(|s| s.to_string()).collect::<HashSet<_>>();
            assert_eq!(res, expect);
        }
        {
            let res = f("".to_string()).into_iter().collect::<HashSet<_>>();
            assert_eq!(res, HashSet::new());
        }

    }

    #[test]
    pub fn test_by_iter() {
        test_method(Solution::by_iter);
    }

    #[test]
    pub fn test_by_generative() {
        test_method(Solution::by_generative);
    }

    #[test]
    pub fn test_by_fold() {
        test_method(Solution::by_fold);
    }
}


