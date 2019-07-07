pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        let mut i = 0;
        let mut j = bytes.len() as i32-1;

        while i < j {
            if !char::from(bytes[i as usize]).is_alphanumeric() {
                i += 1;
                continue;
            }
            if !char::from(bytes[j as usize]).is_alphanumeric() {
                j -= 1;
                continue;
            }

            let a = char::from(bytes[i as usize]).to_ascii_lowercase();
            let b = char::from(bytes[j as usize]).to_ascii_lowercase();
            if a != b {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pal_test() {
        assert_eq!(Solution::is_palindrome("".to_string()), true);
        assert_eq!(Solution::is_palindrome("Am".to_string()), false);
        assert_eq!(Solution::is_palindrome("Aa".to_string()), true);
        assert_eq!(Solution::is_palindrome("A.a".to_string()), true);
        assert_eq!(Solution::is_palindrome("A.. a".to_string()), true);
        assert_eq!(Solution::is_palindrome("ATa".to_string()), true);
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    }
}
