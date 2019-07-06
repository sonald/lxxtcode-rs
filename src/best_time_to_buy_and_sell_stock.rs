pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = i32::max_value();
        let mut sell = 0;
        let mut profit = 0;

        prices.into_iter().for_each(|x| {
            if buy > x {
                buy = x;
                sell = 0;
            } else if sell < x {
                sell = x;
            }
            profit = profit.max((sell - buy).max(0));
        });

        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_test() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 5);
        assert_eq!(Solution::max_profit(vec![7,6,4,3,1]), 0);
        assert_eq!(Solution::max_profit(vec![2,4,1]), 2);
    }
}
