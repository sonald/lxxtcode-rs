use super::utils::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(np) => {
                1 + Self::max_depth(np.borrow().left.clone()).max(Self::max_depth(np.borrow().right.clone()))
            }

            _ => 0
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn method_test() {
        {
            assert_eq!(Solution::max_depth(None), 0);
        }
        {
            let root = TreeNode::new(21);
            assert_eq!(Solution::max_depth(Some(Rc::new(RefCell::new(root)))), 1);
        }
        {
            let mut root = TreeNode::new(1);
            root.insert_left(2);
            root.insert_right(2);
            root.left.as_mut().unwrap().borrow_mut().insert_left(3);
            root.left.as_mut().unwrap().borrow_mut().insert_right(4);
            root.right.as_mut().unwrap().borrow_mut().insert_left(4);
            root.right.as_mut().unwrap().borrow_mut().insert_right(3);

            assert_eq!(Solution::max_depth(Some(Rc::new(RefCell::new(root)))), 3);
        }
        {
            let mut root = TreeNode::new(3);
            root.insert_left(9);
            root.insert_right(20);
            root.right.as_mut().unwrap().borrow_mut().insert_left(15);
            root.right.as_mut().unwrap().borrow_mut().insert_right(7);

            assert_eq!(Solution::max_depth(Some(Rc::new(RefCell::new(root)))), 3);
        }
    }
}

