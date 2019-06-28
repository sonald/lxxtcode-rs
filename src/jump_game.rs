pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.iter().all(|&i| i != 0) {
            return true
        }
        Self::by_backward_dp(nums)
    }

    // slow than backward
    fn by_forward_dp(nums: Vec<i32>) -> bool {
        #[derive(Clone, Copy, PartialEq)]
        enum State {
            Empty,
            Accept,
            Reject,
        }
        use State::*;

        let n = nums.len();
        let mut dp = vec![Empty; n];
        dp[n-1] = Accept;

        fn doit(nums: &Vec<i32>, dp: &mut Vec<State>, i: usize) -> bool {
            if dp[i] != Empty {
                return dp[i] == Accept
            }

            dp[i] = Reject;
            for j in (1..=(nums[i] as usize).min(nums.len()-1-i)).rev() {
                if doit(nums, dp, i+j) {
                    dp[i] = Accept;
                    break;
                }
            }

            dp[i] == Accept
        }

        doit(&nums, &mut dp, 0)
    }

    // O(n*n)
    fn by_backward_dp(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = vec![false; n];
        dp[n-1] = true;

        for i in (0..n-1).rev() {
            for j in 1..=(nums[i] as usize).min(n-1-i) {
                if dp[i+j] {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[0]
    }

    fn by_reach_zeros(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n == 1 { return true; }

        let mut i = n as i32-2;
        let mut zero_pos = i;

        while i >= 0 {
            if nums[i as usize] > 0 {
                i -= 1;
                zero_pos = i;
                continue;
            }

            zero_pos = i;
            i -= 1;
            while i >= 0 {
                if zero_pos-i < nums[i as usize] {
                    zero_pos = i-1;
                    break;
                }

                i -= 1;
            }
        }

        zero_pos < 0
    }

    fn by_greedy(nums: Vec<i32>) -> bool {
        let n = nums.len();

        let mut last_good = n-1;
        for i in (0..n-1).rev() {
            if last_good-i <= nums[i] as usize {
                last_good = i;
            }
        }


        last_good == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn jump_test_backward() {
        method_test(Solution::by_backward_dp);
    }

    #[test]
    fn jump_test_forward() {
        method_test(Solution::by_forward_dp);
    }

    #[test]
    fn jump_test_reach_zeros() {
        method_test(Solution::by_reach_zeros);
    }

    #[test]
    fn jump_test_greedy() {
        method_test(Solution::by_greedy);
    }

    fn method_test(f: impl Fn(Vec<i32>)->bool) {
        {
            let v = vec![0, 1];
            assert_eq!(f(v), false);
        } 
        {
            let v = vec![0];
            assert_eq!(f(v), true);
        } 
        {
            let v = vec![2];
            assert_eq!(f(v), true);
        } 
        {
            let v = vec![2,3,1,1,4];
            assert_eq!(f(v), true);
        } 
        {
            let v = vec![3,2,1,0,4];
            assert_eq!(f(v), false);
        } 
        {
            let v = vec![2,5,0,0];
            assert_eq!(f(v), true);
        } 
        {
            let mut v = vec![0; 100];
            v[0] = 100-1;
            assert_eq!(f(v), true);
        } 
    }


    #[bench]
    fn bench_backward2(b: &mut test::Bencher) {
        b.iter(|| {
            {
                let v = vec![1; 1000];
                Solution::by_backward_dp(v);
            }
        })
    }

    #[bench]
    fn bench_backward(b: &mut test::Bencher) {
        b.iter(|| {
            {
                let mut v = vec![0; 1000];
                v[0] = 1000-1;
                Solution::by_backward_dp(v);
            }
        })
    }

    #[bench]
    fn bench_forward2(b: &mut test::Bencher) {
        b.iter(|| {
            {
                let v = vec![1; 1000];
                Solution::by_forward_dp(v);
            }
        })
    }

    #[bench]
    fn bench_forward(b: &mut test::Bencher) {
        b.iter(|| {
            {
                let mut v = vec![0; 1000];
                v[0] = 1000-1;
                Solution::by_forward_dp(v);
            }
        })
    }
}
