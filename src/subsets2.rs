pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::by_backtrack(nums)
    }

    fn by_backtrack(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut h = HashMap::new();
        let mut res = vec![];

        for i in nums {
            *h.entry(i).or_insert(0) += 1;
        }

        fn backtrack(res: &mut Vec<Vec<i32>>, h: &mut HashMap<i32, i32>, k: usize, sp: &mut Vec<i32>) {
            let val = h.keys().nth(k);
            if val.is_some() {
                let i = *val.unwrap();
                //sp.push(i);
                //backtrack(res, nums, sp);
                //nums.push(sp.pop().unwrap());

                let c = *h.get(&i).unwrap();
                h.entry(i).and_modify(|p| *p = 0);
                for j in 0..=c as usize {
                    (0..j).for_each(|_| sp.push(i));
                    backtrack(res, h, k+1, sp);
                    (0..j).for_each(|_| { sp.pop(); });
                }
                h.entry(i).and_modify(|p| *p = c);
            } else {
                res.push(sp.clone());
            }
        }

        backtrack(&mut res, &mut h, 0, &mut vec![]);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method_test(f: impl Fn(Vec<i32>) -> Vec<Vec<i32>>) {
        {
            let v = vec![1,1,2,2];
            let mut expect = vec!{
                vec![],
                vec![1],
                vec![1,2],
                vec![1,2,2],
                vec![1,1],
                vec![1,1,2],
                vec![1,1,2,2],
                vec![2],
                vec![2,2],
            };

            let mut res = f(v);
            res.iter_mut().for_each(|v| v.sort());
            res.sort();
            expect.sort();
            assert_eq!(res, expect);
        }
        {
            let v = vec![1,2,2];
            let mut expect = vec!{
                vec![1],
                vec![2],
                vec![1,2,2],
                vec![1,2],
                vec![2,2],
                vec![]
            };

            let mut res = f(v);
            res.iter_mut().for_each(|v| v.sort());
            res.sort();
            expect.sort();
            assert_eq!(res, expect);
        }
        {
            let v = vec![1,2,3];
            let mut expect = vec!{
                vec![3],
                vec![1],
                vec![2],
                vec![1,2,3],
                vec![1,3],
                vec![2,3],
                vec![1,2],
                vec![]
            };

            let mut res = f(v);
            res.iter_mut().for_each(|v| v.sort());
            res.sort();
            expect.sort();
            assert_eq!(res, expect);
        }
    }


    #[test]
    fn power_test() {
        method_test(Solution::by_backtrack);
    }
}

