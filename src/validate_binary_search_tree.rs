use super::utils::*;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn validate(root: Option<Rc<RefCell<TreeNode>>>, max: i64, min: i64) -> bool {
            match root {
                None => true,
                Some(inner) => {
                    let val = inner.borrow().val as i64;
                    val < max && val > min &&
                    validate(inner.borrow().left.clone(), val, min) &&
                        validate(inner.borrow().right.clone(), max, val)
                }
            }

        }

        validate(root, i64::max_value(), i64::min_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_sol1() {
        {
            let root = TreeNode::new(2147483647);
            assert_eq!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))), true);
        }
        {
            assert_eq!(Solution::is_valid_bst(None), true);
        }
        {
            let mut root = TreeNode::new(1);
            root.insert_right(2);
            root.right.as_mut().unwrap().borrow_mut().insert_right(3);

            assert_eq!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))), true);
        }
        {
            let mut root = TreeNode::new(1);
            root.insert_right(2);
            root.right.as_mut().unwrap().borrow_mut().insert_left(3);

            assert_eq!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))), false);
        }
        {
            let mut root = TreeNode::new(1);
            root.insert_left(2);
            root.insert_right(3);
            root.left.as_mut().unwrap().borrow_mut().insert_left(4);
            root.right.as_mut().unwrap().borrow_mut().insert_right(5);

            assert_eq!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))), false);
        }
        {
            let root = build_tree!(5,2,3,4,1,6,7);
            assert_eq!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))), true);
        }
        {
            let mut root = TreeNode::new(5);
            root.insert_left(2);
            root.insert_right(9);
            root.left.as_mut().unwrap().borrow_mut().insert_left(1);
            root.right.as_mut().unwrap().borrow_mut().insert_right(11);

            assert_eq!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))), true);
        }
        {
            let mut root = TreeNode::new(10);
            root.insert_left(5);
            root.insert_right(15);
            root.right.as_mut().unwrap().borrow_mut().insert_left(6);
            root.right.as_mut().unwrap().borrow_mut().insert_right(20);

            assert_eq!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))), false);
        }
    }
}

