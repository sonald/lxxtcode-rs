pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut k = n+m-1;
        let mut i = m-1;
        let mut j = n-1;
        while i >= 0 && j >= 0 {
            nums1[k as usize] = if nums1[i as usize] > nums2[j as usize] {
                i -= 1;
                nums1[(i+1) as usize]
            } else {
                j -= 1;
                nums2[(j+1) as usize]
            };

            k -=1;
        }

        if j >= 0 {
            (0..=j).for_each(|x| nums1[x as usize] = nums2[x as usize]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method_test(f: impl Fn(&mut Vec<i32>, i32, &mut Vec<i32>, i32)) {
        {
            let mut v1 = vec![1,2,3,0,0,0];
            let mut v2 = vec![2,5,6];
            let out = vec![1,2,2,3,5,6];
            f(&mut v1, 3, &mut v2, 3);
            assert_eq!(v1, out);
        }
        {
            let mut v1 = vec![1,2,0,0,0,0];
            let mut v2 = vec![3,4,5,6];
            let out = vec![1,2,3,4,5,6];
            f(&mut v1, 2, &mut v2, 4);
            assert_eq!(v1, out);
        }
        {
            let mut v1 = vec![7,8,0,0,0,0];
            let mut v2 = vec![3,4,5,6];
            let out = vec![3,4,5,6,7,8];
            f(&mut v1, 2, &mut v2, 4);
            assert_eq!(v1, out);
        }
    }

    #[test]
    fn sorted_test() {
        method_test(Solution::merge); 
    }
}
