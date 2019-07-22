pub struct Solution;
impl Solution {
    //NOTE: nice idea from others: use fast & slow pointers
    pub fn is_happy(mut n: i32) -> bool {
        use std::collections::HashSet;
        let mut m = HashSet::new();
        while n != 1 && !m.contains(&n) {
            m.insert(n);

            let mut sum = 0;
            while n > 0 {
                sum += (n % 10).pow(2);
                n /= 10;
            }

            n = sum;
        }

        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_test() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(20), false);
        assert_eq!(Solution::is_happy(2), false);
    }
}
