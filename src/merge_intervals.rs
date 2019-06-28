pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::by_sort(intervals)
    }

    pub fn by_sort(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|v| v[0]);
        intervals.into_iter().fold(vec![], |mut acc, v| {
            if acc.len() == 0 {
                acc.push(v);
            } else {
                let p = acc.last_mut().unwrap();
                if p[1] >= v[0] {
                    p[1] = v[1].max(p[1]);
                } else {
                    acc.push(v);
                }
            }
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_test() {
        {
            let v = [[1,3],[2,6],[8,10],[15,18]].iter().map(|s| s.to_vec()).collect();
            let expect = [[1,6],[8,10],[15,18]].iter().map(|s| s.to_vec()).collect::<Vec<_>>();
            assert_eq!(Solution::merge(v), expect);
        }
        {
            let v = [[1,4],[2,3]].iter().map(|s| s.to_vec()).collect();
            let expect = vec![vec![1,4]];
            assert_eq!(Solution::merge(v), expect);
        }
        {
            let v = [[1,3],[3,6]].iter().map(|s| s.to_vec()).collect();
            let expect = vec![vec![1,6]];
            assert_eq!(Solution::merge(v), expect);
        }
    }
}
