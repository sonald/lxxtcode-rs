pub struct Solution;

impl Solution {
    //TODO: KMP, BM
    pub fn str_str(haystack: String, needle: String) -> i32 {
        Self::naive(haystack, needle)
    }

    pub fn naive(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }
        if haystack.len() < needle.len() {
            return -1;
        }
        for i in 0..=haystack.len() - needle.len() {
            let sub = &haystack[i..];
            if sub.chars().zip(needle.chars()).all(|(a, b)| a == b) {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::implement_strstr::Solution;

    fn test_method(f: impl Fn(String, String) -> i32) {
        assert_eq!(f("a".to_string(), "a".to_string()), 0);
        assert_eq!(f("".to_string(), "a".to_string()), -1);
        assert_eq!(f("hello".to_string(), "".to_string()), 0);
        assert_eq!(f("needle".to_string(), "ee".to_string()), 1);
        assert_eq!(f("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(f("bbbbcababb".to_string(), "bba".to_string()), -1);
    }

    #[test]
    fn test_naive() {
        test_method(Solution::naive)
    }
}
