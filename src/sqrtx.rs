pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        Self::newton_method(x as f64).floor() as i32
    }

    pub fn newton_method(x: f64) -> f64 {
        let mut p = x / 2.0; // init guess
        while (p*p - x).abs() > 0.000001 {
            //eprintln!("x = {}, p = {}, abs = {}", x, p, (p*p-x).abs());
            p = (p + x / p) * 0.5;
        }

        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn origin(x: i32) -> i32 {
        (x as f32).sqrt().floor() as i32
    }

    #[test]
    fn method_test() {
        assert_eq!(origin(1), 1); 
        assert_eq!(origin(2), 1); 
        assert_eq!(origin(3), 1); 
        assert_eq!(origin(4), 2); 
        assert_eq!(origin(5), 2); 
        assert_eq!(origin(7), 2); 
        assert_eq!(origin(8), 2); 
        assert_eq!(origin(9), 3); 
        assert_eq!(origin(10), 3); 

        Solution::my_sqrt(100);

        for i in 1..1000 {
            assert_eq!(origin(i), Solution::my_sqrt(i));
        }
        Solution::my_sqrt(2147395599);
    }
}
