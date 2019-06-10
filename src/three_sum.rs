use std::vec::Vec;

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Solution::basic(nums)
    }

    //slowest as corrctness foundation
    fn basic(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut res = HashSet::new();

        for i in 0..nums.len() {
            for j in (i+1)..nums.len() {
                for k in (j+1)..nums.len() {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        let mut v = vec!(nums[i], nums[j], nums[k]);
                        v.sort();
                        res.insert(v);
                    }
                }
            }
        }

        res.into_iter().collect()
    }

    fn by_sort1(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut res = HashSet::new();
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        for i in 0..n {
            for j in (i+1)..n {
                let sum2 = nums[i] + nums[j];
                let mut l = j+1;
                let mut r = n-1;
                while l <= r {
                    let m = l + (r-l)/2;
                    //eprintln!("---  ({}, {}, {})", l, r, m);
                    match sum2 + nums[m] {
                        x if x == 0 => {
                            let mut v = vec!(nums[i], nums[j], nums[m]);
                            res.insert(v);
                            break;
                        },
                        x if x > 0 => {
                            r = m-1;
                        },
                        x if x < 0 => {
                            l = m+1;
                        },
                        _ => { unreachable!(); }
                    }
                }
            }
        }

        res.into_iter().collect()
    }

    // fast, could be faster if eliminate duplicates without HashSet
    fn by_sort_and_2sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let mut res = HashSet::new();
        nums.sort();

        for i in 0..nums.len() {
            let target = -nums[i];
            let mut j = i+1;
            let mut k = nums.len()-1;

            while j < k  {
                if nums[j] + nums[k] == target {
                    res.insert(vec![nums[i], nums[j], nums[k]]);
                    j+=1; k-=1;

                } else if nums[j] + nums[k] < target {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        res.into_iter().collect()
    }

    //slow
    fn by_2sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let mut res = HashSet::new();

        for i in 0..nums.len() {
            let target = -nums[i];
            let mut h = HashSet::new();

            for j in (i+1)..nums.len() {
                let v = nums[j];
                let u = target - v;
                match h.get(&u) {
                    Some(_) => {
                        let mut vec = vec![nums[i], u, v];
                        vec.sort();
                        res.insert(vec);
                    },
                    _ => {
                        h.insert(v);
                    }
                }
            }
        }

        res.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::*;
    use super::*;
    use std::time::*;
    use std::collections::HashSet;

    fn compare_2dvec(mut s1: Vec<Vec<i32>>, mut s2: Vec<Vec<i32>>) -> bool {
        s1.iter_mut().for_each(|v| {v.sort(); });
        s1.sort();
        s2.iter_mut().for_each(|v| {v.sort(); });
        s2.sort();

        eprintln!("s1 = {:?}\ns2 = {:?}", s1, s2);
        s1 == s2
    }


    fn test_method<F>(f: F) where F: Fn(Vec<i32>) -> Vec<Vec<i32>> {
        {
            let v = vec![];
            let expect = vec![];
            assert!(compare_2dvec(f(v), expect));
        }
        {
            let v = vec![1];
            let expect = vec![];
            assert!(compare_2dvec(f(v), expect));
        }
        {
            let v = vec![1, -1];
            let expect = vec![];
            assert!(compare_2dvec(f(v), expect));
        }
        {
            let v = vec![-1, 0, 1, 2, -1, -4, 4];
            let expect = vec![vec![-1, 0, 1], vec![-1, -1, 2], vec![-4, 0, 4]];
            assert!(compare_2dvec(f(v), expect));
        }
        {
            let v = vec![-1, 0, 1, 2, -1, -4];
            let expect = vec![vec![-1, 0, 1], vec![-1, -1, 2]];
            assert!(compare_2dvec(f(v), expect));
        }
        {
            let v = vec![-1, 0, 0, 1, 2, -1, 2, -1, -4];
            let expect = vec![vec![-1, 0, 1], vec![-1, -1, 2], vec![-4, 2, 2]];
            assert!(compare_2dvec(f(v), expect));
        }
        {
            let v = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            let expect = vec![vec![0, 0, 0]];
            assert!(compare_2dvec(f(v), expect));
        }
        { // slow
            //let v = include!("./3sumdata.txt");
            //let expect = vec![vec![-1, 0, 1], vec![-1, -1, 2]];
            //f(v);
        }

    }

    #[test]
    pub fn test_3sum() {
        test_method(Solution::basic);
    }

    #[test]
    pub fn test_by_sort_and_2sum() {
        test_method(Solution::by_sort_and_2sum);
        {
            let now = Instant::now();
            let v = include!("./3sumdata.txt");
            let res = Solution::by_sort_and_2sum(v);
            eprintln!("load time : {}", now.elapsed().as_millis());

            let expect = vec![vec![-1, 0, 1], vec![-1, -1, 2]];
            //assert!(compare_2dvec(res, expect));
        }

    }

    #[test]
    pub fn test_by_2sum() {
        test_method(Solution::by_2sum);
    }

    #[test]
    pub fn test_by_sort1() {
        test_method(Solution::by_sort1);
    }

    #[bench]
    pub fn bench_sort1(b: &mut Bencher) {
        let v = include!("./3sumdata.txt");
        //b.iter(|| Solution::by_sort1(v.clone()));
    }
}

