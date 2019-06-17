pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::by_recur(n)
    }

    //TODO: closure number ?
    fn by_closure_number(n: i32) -> Vec<String> {
        vec![]
    }

    fn by_recur(n: i32) -> Vec<String> {
        use std::collections::HashSet;
        let mut res = HashSet::new();
        if n == 1 {
            return vec!["()".to_string()];
        }

        for s in Self::by_recur(n - 1) {
            let pos = s
                .chars()
                .enumerate()
                .filter_map(|(i, c)| match c == '(' {
                    true => Some(i),
                    false => None,
                })
                .collect::<Vec<_>>();
            for i in pos {
                let mut t = s.clone();
                t.insert_str(i, "()");
                res.insert(t);

                let mut t = s.clone();
                t.insert_str(i + 1, "()");
                res.insert(t);
            }
        }

        res.into_iter().collect()
    }

    fn by_backtrack(n: i32) -> Vec<String> {
        let mut res = vec![];
        Self::_backtrack(&mut res, &mut "".to_string(), 0, 0, n);

        res
    }

    fn _backtrack(v: &mut Vec<String>, s: &mut String, open: i32, close: i32, n: i32) {
        if close == n {
            v.push(s.clone());
            return;
        }

        if open < n {
            s.push('(');
            Self::_backtrack(v, s, open + 1, close, n);
            s.pop();
        }

        if close < open {
            s.push(')');
            Self::_backtrack(v, s, open, close + 1, n);
            s.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::generate_parentheses::Solution;
    use std::collections::HashSet;
    use std::hash::Hash;

    fn test_method<F>(f: F)
    where
        F: Fn(i32) -> Vec<String>,
    {
        {
            let res = f(3).into_iter().collect::<HashSet<_>>();
            let expect = ["((()))", "(()())", "(())()", "()(())", "()()()"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>();

            assert_eq!(res, expect);
        }
    }

    #[test]
    fn test_recurse() {
        test_method(Solution::by_recur)
    }

    #[test]
    fn test_backtrack() {
        test_method(Solution::by_backtrack)
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_backtrack(b: &mut Bencher) {
        b.iter(|| {
            Solution::by_backtrack(9);
        })
    }

    #[bench]
    fn bench_recur(b: &mut Bencher) {
        b.iter(|| {
            Solution::by_recur(9);
        })
    }
}
