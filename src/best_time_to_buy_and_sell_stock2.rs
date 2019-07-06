pub struct Solution;

// two ideas
impl Solution {
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 { return 0; }
        let mut min = prices[0];
        let mut curr = 0;
        let mut profit = 0;

        for i in 1..prices.len() {
            if prices[i-1] >= prices[i] {
                if curr > 0 {
                    profit += curr - min;
                    curr = 0;
                }
                min = prices[i];
            } else {
                curr = prices[i];
            }
        }

        if curr > 0 {
            profit += curr - min;
        }

        profit
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.windows(2).fold(0, |acc, pair| {
            acc + (pair[1] - pair[0]).max(0)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_test() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 7);
        assert_eq!(Solution::max_profit(vec![7,6,4,3,1]), 0);
        assert_eq!(Solution::max_profit(vec![1,2,3,4,5]), 4);
        assert_eq!(Solution::max_profit(vec![]), 0);
    }
}

