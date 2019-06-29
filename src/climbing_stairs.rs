pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        Self::by_dp(n)
    }

    pub fn by_dp(n: i32) -> i32 {
        let n = n as usize;
        let mut d1 = 1;
        let mut d2 = 1;

        (1..n).for_each(|_| {
            let d = d2 + d1;
            d1 = d2;
            d2 = d;
        });

        d2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method_test(f: impl Fn(i32)->i32) {
        assert_eq!(f(1), 1);
        assert_eq!(f(2), 2);
        assert_eq!(f(3), 3);
        assert_eq!(f(4), 5);
        assert_eq!(f(5), 8);
        assert_eq!(f(6), 13);
    }

    #[test]
    fn dp_test() {
        method_test(Solution::by_dp);
    }
}

