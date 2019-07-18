pub struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        Solution::sol1(nums)
    }

    pub fn sol1(mut nums: Vec<i32>) -> String {
        let mut all0 = true;

        nums.sort_by(|&a, &b| {
            let a = a as i64;
            let b = b as i64;

            let mut radix = if a == 0 { 10 } else { 1 };
            while a / radix > 0 {
                radix *= 10;
            }
            let ba = b * radix + a;

            let mut radix = if b == 0 { 10 } else { 1 };
            while b / radix > 0 {
                radix *= 10;
            }
            let ab = a * radix + b;

            if all0 { all0 = ab == 0; }

            ba.cmp(&ab)
        });

        if all0 { return "0".to_string(); }

        nums.into_iter().fold(String::new(), |mut res, i| {
            res.push_str(&i.to_string());
            res
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_number_test() {
        
        assert_eq!(Solution::largest_number(vec![0, 0]), "0".to_string());
        assert_eq!(Solution::largest_number(vec![999999998,999999997,999999999]),
            "999999999999999998999999997".to_string());
        assert_eq!(Solution::largest_number(vec![121, 12]), "12121".to_string());
        assert_eq!(Solution::largest_number(vec![34, 3]), "343".to_string());
        assert_eq!(Solution::largest_number(vec![30, 3]), "330".to_string());
        assert_eq!(Solution::largest_number(vec![3, 0, 2]), "320".to_string());
        assert_eq!(Solution::largest_number(vec![32, 3]), "332".to_string());
        assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_string());
        assert_eq!(Solution::largest_number(vec![3,30,34,5,9]), "9534330".to_string());
    }
}
