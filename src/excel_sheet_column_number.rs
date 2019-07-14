pub struct Solution;

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        Self::forward(s)
    }

    pub fn forward(s: String) -> i32 {
        s.chars().fold(0, |res, c| {
            res * 26  + (c as i32 - 64)
        })
    }

    pub fn backward(s: String) -> i32 {
        let mut res = 0;
        let mut radix = 1;
        for c in s.as_bytes().iter().rev() {
            let i = c - 64;
            res += i as i32 * radix;
            radix *= 26;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excel_test() {
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
        assert_eq!(Solution::title_to_number("B".to_string()), 2);
        assert_eq!(Solution::title_to_number("C".to_string()), 3);
        assert_eq!(Solution::title_to_number("Z".to_string()), 26);
        assert_eq!(Solution::title_to_number("AA".to_string()), 27);
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
        assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
    }
}
