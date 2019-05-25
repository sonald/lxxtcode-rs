#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() < 1 { return String::new(); }

        //Solution::base(strs)
        Solution::horizontal_scan(strs)
    }
    fn horizontal_scan(strs: Vec<String>) -> String {
        let mut prefix= strs[0].len();

        for i in 0..strs.len()-1 {
            prefix = strs[i].chars().zip(strs[i+1].chars()).map(|(x, y)| x == y).take_while(|&v| v).count().min(prefix);
        }

        strs[0][..prefix].to_string()
    }

    // vertical scan
    fn base(strs: Vec<String>) -> String {
        let mut i = 0;
        'out: loop {
            let ch = strs[0].chars().nth(i);
            for s in &strs {
                if ch == None || ch != s.chars().nth(i) {
                    break 'out
                }
            }

            i += 1;
        }

        strs[0][..i].to_string()
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[test]
    pub fn test_base() {
        {
            let strs: Vec<String> = ["flight", "flower", "flow"].iter().map(|s| s.to_string()).collect();
            assert_eq!(Solution::longest_common_prefix(strs), "fl".to_string());
        }
        {
            let strs = ["fight", "flower", "low"].iter().map(|s| s.to_string()).collect();
            assert_eq!(Solution::longest_common_prefix(strs), "".to_string());
        }
        {
            let strs = ["fight"].iter().map(|s| s.to_string()).collect();
            assert_eq!(Solution::longest_common_prefix(strs), "fight".to_string());
        }
        {
            let strs = vec![];
            assert_eq!(Solution::longest_common_prefix(strs), "".to_string());
        }
    }

    #[bench]
    pub fn bench_hort(b: &mut Bencher) {
        b.iter(|| {
            let strs: Vec<String> = ["flight", "flower", "flow"].iter().map(|s| s.to_string()).collect();
            Solution::horizontal_scan(strs);
        })
    }

    #[bench]
    pub fn bench_base(b: &mut Bencher) {
        b.iter(|| {
            let strs: Vec<String> = ["flight", "flower", "flow"].iter().map(|s| s.to_string()).collect();
            Solution::base(strs);
        })
    }
}


