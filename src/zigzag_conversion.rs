//! see [here](https://leetcode.com/problems/zigzag-conversion/)
//!

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        //Solution::by_bytes(s, num_rows)
        Solution::convert_chars(s, num_rows)
    }

    fn convert_chars(s: String, n: i32) -> String {
        if n < 2 { return s }

        let mut reversed:i32 = -1;
        let mut res = vec![String::new(); n as usize];
        let mut r:i32 = 0;

        for ch in s.chars() {
            res[r as usize].push(ch);
            if r == 0 || r == (n - 1) { reversed = -reversed; }
            r += reversed;
        }


        res.join("")
    }

    fn by_bytes(s: String, num_rows: i32) -> String {
        if num_rows == 1 { return s; }

        let b = s.as_bytes();
        std::str::from_utf8(&Solution::convert_bytes(b, num_rows)).map(|b| b.to_string()).unwrap()
    }

    fn convert_bytes(s: &[u8], n: i32) -> Vec<u8> {
        let mut reversed = true;
        let mut res = vec![vec![]; n as usize];
        let mut r = 0;

        for i in 0..s.len() {
            println!("#{}, r = {}, reversed = {}", i, r, reversed);
            res[r].push(s[i]);

            if r == 0 || r == (n as usize - 1) { reversed = !reversed; }

            r = if reversed { r-1 } else { r+1 };
        }

        res.into_iter().flat_map(|v| v).collect()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    extern crate test;
    use super::*;
    use test::*;

    #[test]
    fn test_primitive() {
        color_backtrace::install();

        assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 1), "PAYPALISHIRING".to_string());

        //Input: s = "PAYPALISHIRING", numRows = 3
        //Output: "PAHNAPLSIIGYIR"
        assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
        //Input: s = "PAYPALISHIRING", numRows = 4
        //Output: "PINALSIGYAHRPI"
        assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI".to_string());
    }
}

