pub struct Solution;

impl Solution {
    pub fn fraction_to_decimal(no: i32, de: i32) -> String {
        Self::by_hash(no as i64, de as i64)
    }

    pub fn by_hash(no: i64, de: i64) -> String {
        use std::collections::HashMap;

        let neg = (no < 0 && de > 0) || (de < 0 && no > 0);
        let (no, de) = (no.abs(), de.abs());
        let mut res = String::new();

        if neg { res.push('-'); }
        res.push_str(&(no / de).to_string());

        let mut m = HashMap::new();
        let mut no = no % de;
        if no > 0 { res.push('.'); }

        m.entry(no).or_insert(res.len());
        while no > 0 {
            no *= 10;
            let d = no / de;
            res.push(char::from(d as u8 + 48));
            no = no % de;

            match m.get(&no) {
                Some(&i) => {
                    res.insert(i, '(');
                    res.push(')');
                    break
                },
                _ => {
                    m.entry(no).or_insert(res.len());
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn fraction_test() {
        assert_eq!(Solution::fraction_to_decimal(-1, -2147483648), "0.0000000004656612873077392578125".to_string());

        assert_eq!(Solution::fraction_to_decimal(1, 2), "0.5".to_string());
        assert_eq!(Solution::fraction_to_decimal(1, 6), "0.1(6)".to_string());
        assert_eq!(Solution::fraction_to_decimal(2, 1), "2".to_string());
        assert_eq!(Solution::fraction_to_decimal(2, 3), "0.(6)".to_string());
        assert_eq!(Solution::fraction_to_decimal(1, 37), "0.(027)".to_string());
        assert_eq!(Solution::fraction_to_decimal(1, 7), "0.(142857)".to_string());
        assert_eq!(Solution::fraction_to_decimal(1000, 7), "142.(857142)".to_string());
        assert_eq!(Solution::fraction_to_decimal(1000, 37), "27.(027)".to_string());
        assert_eq!(Solution::fraction_to_decimal(1, 70000), "0.0000(142857)".to_string());
        assert_eq!(Solution::fraction_to_decimal(1, 377), "0.(002652519893899204244031830238726790450928381962864721485411140583554376657824933687)".to_string());
        assert_eq!(Solution::fraction_to_decimal(-1, 6), "-0.1(6)".to_string());
        assert_eq!(Solution::fraction_to_decimal(-1, -6), "0.1(6)".to_string());
    }

    #[bench]
    fn bench_frac(b: &mut test::Bencher) {
        b.iter(|| {
            Solution::fraction_to_decimal(1, 3777);
        })
    }
}
