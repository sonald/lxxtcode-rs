pub struct Solution;
// 测试编码格式
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        Self::by_iterate(nums)
    }

    pub fn by_iterate(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut newi = 1;

        for i in 1..nums.len() {
            if nums[newi - 1] != nums[i] {
                nums[newi] = nums[i];
                newi += 1;
            }
        }

        // nums.truncate(newi);

        newi as i32
    }

    pub fn by_backward(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut i = nums.len() - 2;

        loop {
            if i == 0 {
                break;
            }
            if nums[i + 1] == nums[i] {
                nums.remove(i + 1);
            }
            i -= 1;
        }

        if nums[0] == nums[1] {
            nums.remove(1);
        }

        nums.len() as i32
    }

    pub fn by_forward(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut i = 1;

        loop {
            if i >= nums.len() {
                break;
            }

            if nums[i - 1] == nums[i] {
                nums.remove(i);
            } else {
                i += 1;
            }
        }

        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::remove_duplicates_from_sorted_array::Solution;

    #[test]
    pub fn test() {
        let mut v = vec![0, 1, 1, 2, 3, 3, 4, 5, 5];
        let res = Solution::remove_duplicates(&mut v);
        assert_eq!(res, 6);
        for i in 0..6 {
            assert_eq!(v[i], i as i32);
        }
    }
}
