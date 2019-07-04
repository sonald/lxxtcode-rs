use super::utils::*;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut sp = vec![];
        let mut res = vec![];

        let mut np = root;

        loop {
            while np.is_some() {
                sp.push(np.clone());
                np = np.unwrap().borrow().left.clone();
            }

            if sp.len() == 0 {
                break
            }    

            np = sp.pop().unwrap();
            res.push(np.as_ref().unwrap().borrow().val);

            np = np.unwrap().borrow().right.clone();
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_sol1() {
        {
            assert_eq!(Solution::inorder_traversal(None), vec![]);
        }
        {
            let mut root = TreeNode::new(1);
            root.insert_right(2);
            root.right.as_mut().unwrap().borrow_mut().insert_left(3);

            let res = vec![1,3,2];
            assert_eq!(Solution::inorder_traversal(Some(Rc::new(RefCell::new(root)))), res);
        }
        {
            let mut root = TreeNode::new(1);
            root.insert_left(2);
            root.insert_right(3);
            root.left.as_mut().unwrap().borrow_mut().insert_left(4);
            root.right.as_mut().unwrap().borrow_mut().insert_right(5);

            let res = vec![4,2,1,3,5];
            assert_eq!(Solution::inorder_traversal(Some(Rc::new(RefCell::new(root)))), res);
        }
        {
            let root = build_tree!(5,2,3,4,1,6,7);
            let res = vec![1,2,3,4,5,6,7];
            assert_eq!(Solution::inorder_traversal(Some(Rc::new(RefCell::new(root)))), res);
        }
    }
}
