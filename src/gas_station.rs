pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        Self::by_greedy(gas, cost)
    }

    fn by_bruteforce(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        for i in 0..gas.len() {
            let mut j = (i+1) % gas.len();
            let mut left = gas[i] - cost[i];
            loop {
                if left < 0 {
                    break
                }

                if j == i {
                    return i as i32;
                }

                left += gas[j] - cost[j];
                j = (j+1) % gas.len();
            }
        }   

        -1
    }

    fn by_greedy(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut max = i32::min_value();
        let mut target = -1;
        let mut bottom = -1;
        let mut total = 0;
        gas.iter().zip(cost.iter()).enumerate().map(|(i, (g, c))| (i, g-c))
            .for_each(|(i, k)| {
                total += k;
                if k < 0 {
                    bottom = i as i32;
                    left = 0;
                } else {
                    left += k;
                    if left > max {
                        max = left;
                        target = ((bottom as usize + 1) % gas.len()) as i32;
                    }
                }
            });

        if total >= 0 {
            target
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method_test(f: impl Fn(Vec<i32>, Vec<i32>) -> i32) {
        {
            let gas  = vec![1,2,3,4,5];
            let cost = vec![3,4,5,1,2];
            assert_eq!(f(gas, cost), 3);
        }
        {
            let gas  = vec![2,3,4];
            let cost = vec![3,4,3];
            assert_eq!(f(gas, cost), -1);
        }
        {
            let gas  = vec![5,8,2,8];
            let cost = vec![6,5,6,6];
            assert_eq!(f(gas, cost), 3);
        }
        {
            let gas  = vec![1,2,3,4];
            let cost = vec![2,3,4,5];
            assert_eq!(f(gas, cost), -1);
        }
        {
            let gas  = vec![1,3,3,9];
            let cost = vec![3,2,5,2];
            assert_eq!(f(gas, cost), 3);
        }
        {
            let gas  = vec![7,1,0,11,4];
            let cost = vec![5,9,1,2,5];
            assert_eq!(f(gas, cost), 3);
        }
    }

    #[test]
    fn gas_test() {
        //method_test(Solution::by_bruteforce);
        method_test(Solution::by_greedy);
    }
}
