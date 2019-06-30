pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        Self::by_count_sort(nums)
    }

    pub fn by_split_arrange(nums: &mut Vec<i32>) {
    }

    pub fn by_count_sort(nums: &mut Vec<i32>) {
        let mut counts = [0; 3];
        nums.iter().for_each(|&x| counts[x as usize] += 1);
        for (i, p) in nums.iter_mut().enumerate() {
            if i < counts[0] {
                *p = 0;
            } else if i < counts[0] + counts[1] {
                *p = 1;
            } else {
                *p = 2;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method_test(f: impl Fn(&mut Vec<i32>)) {
        {
            let mut v = vec![2,0,2,1,1,0];
            let expect = vec![0,0,1,1,2,2];
            f(&mut v);
            assert_eq!(v, expect);
        }
    }
    
    #[test]
    fn sort_test() {
        method_test(Solution::by_count_sort);
    }
}
