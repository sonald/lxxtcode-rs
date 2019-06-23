pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        Self::by_iter(x, n) 
    }


    fn by_iter(x: f64, n: i32) -> f64 {
        let mut res = 1.0;
        let mut m = n.abs() as u32;
        let mut k = x;

        while m > 0 {
            if m & 1 == 1 {
                res *= k;
            }

            k *= k;
            m = m >> 1;
        }

        if n < 0 {
            1.0f64 / res
        } else {
            res
        }
    }

    fn by_base(x: f64, n: i32) -> f64 {
        let mut res = 1.0;

        (0..n.abs()).for_each(|_| { res *= x } );

        if n < 0 {
            1.0f64 / res
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_eq(x: f64, y: f64) {
        let abs_difference = (x - y).abs();
        //eprintln!("{} - {} = {}",x, y, abs_difference);
        assert!(abs_difference < 1e-8);
    }

    fn method_test(f: impl Fn(f64, i32)->f64) {
        check_eq(f(3.192000, 1), 3.192000);
        check_eq(f(2.0, 4), 16.0);
        check_eq(f(-2.0, 5), -32.0);
        check_eq(f(2.0000000, 10), 1024.000000);
        check_eq(f(2.10000, 3), 9.26100);
        check_eq(f(-2.10000, 3), -9.26100);
        check_eq(f(2.00000, -2), 0.25000);
        check_eq(f(4.00000, 0), 1.000);

        for n in -100..100 {
            check_eq(f(1.12, n), 1.12f64.powf(n as f64));
        }
    }

    #[test]
    fn pow_test() {
        method_test(Solution::by_base);
        method_test(Solution::by_iter);
    }
}
