use super::utils::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dnq(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            let m = (nums.len() - 1) / 2;
            let mut root = TreeNode::new(nums[m as usize]);
            if m > 0 {
                root.left = dnq(&nums[0..m]);
            }

            if m + 1 < nums.len() {
                root.right = dnq(&nums[m + 1..]);
            }
            Some(Rc::new(RefCell::new(root)))
        }

        match nums.len() == 0 {
            true => None,
            _ => dnq(&nums),
        }
    }
}
