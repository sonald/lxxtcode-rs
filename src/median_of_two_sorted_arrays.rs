//! see [here](https://leetcode.com/problems/
//!

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (l1, l2) = (nums1.len(), nums2.len());
        let (k1, k2) = ((l1+l2-1)/2, (l1+l2)/2);

        if l1 == 0 {
            return (nums2[k1] + nums2[k2]) as f64 / 2.0
        } else if l2 == 0 {
            return (nums1[k1] + nums1[k2]) as f64 / 2.0
        }

        let (v1, v2) = if nums1[0] > nums2[0] {
            (&nums2, &nums1)
        } else {
            (&nums1, &nums2)
        };
        let (l1, l2) = (v1.len(), v2.len());
        
        //let len = l1 + l2;
        let mut res = (0, 0);

        if v1.last().unwrap() <= &v2[0] {
            if k1 < l1 {
                res.0 = v1[k1];
            } else {
                res.0 = v1[k1-l1];
            }

            if k2 < l1 {
                res.1 = v1[k2];
            } else {
                res.1 = v2[k2-l1];
            }
        } else if v1.last() < v2.last() {
        } else {
        }


        (res.0 + res.1) as f64 / 2.0
    }

    // precond: v1[0] <= v2[0]
    fn find_in_sorted_arrays(v1: &[i32], v2: &[i32], k: usize) -> i32 {
        let (l1, l2) = (v1.len(), v2.len());
        
        let mut res = 0;
        let (m1, m2) = (l1/2, l2/2);

        match v2.binary_search(&v1[m1]) {
            Ok(p) => {
                if k == m1 + p {
                    return v1[m1];
                } else if k < m1 + p {
                    return Solution::find_in_sorted_arrays(&v1[0..m1], &v2[0..p], k);
                } else {
                    return Solution::find_in_sorted_arrays(&v1[m1..l1], &v2[p..l2], k-m1-p-1);
                }
            },

            Err(p) => {
            }
        }

        res
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    extern crate test;
    use super::*;
    use test::*;

    fn gen_random_string() -> String {
        use rand::random;
        (1..100).map(|_| char::from(random::<u8>() % 128)).collect()
    }

    #[test]
    fn test_primitive() {
    }

    #[bench]
    fn bench_expand(b: &mut Bencher) {
        b.iter(|| {
        })
    }
}
