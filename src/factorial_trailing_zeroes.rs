pub struct Solution;

impl Solution {
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            res += n / 5;
            n = n / 5;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trailing_zeroes_test() {
        for i in 1..=20 {
            let res = Solution::trailing_zeroes(i);
            if i < 5 {
                assert_eq!(res, 0);
            } else if i < 10 {
                assert_eq!(res, 1);
            } else if i < 15 {
                assert_eq!(res, 2);
            } else if i < 20 {
                assert_eq!(res, 3);
            } else {
                assert_eq!(res, 4);
            }
        }

        assert_eq!(Solution::trailing_zeroes(30), 7);
        assert_eq!(Solution::trailing_zeroes(625), 156);

    }
}
