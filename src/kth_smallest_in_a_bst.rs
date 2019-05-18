use std::cell::RefCell;
use std::fmt::{Display, Error, Formatter};
use std::rc::Rc;
use std::result::Result;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "(")?;
        if let Some(np) = self.left.as_ref() {
            write!(f, "{} ", np.borrow())?;
        }

        write!(f, "{}", self.val)?;

        if let Some(np) = self.right.as_ref() {
            write!(f, " {}", np.borrow())?;
        }

        write!(f, ")")
    }
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    #[inline]
    pub fn insert(&mut self, val: i32) {
        match self.val > val {
            true => match self.left.as_mut() {
                None => self.left = Some(Rc::new(RefCell::new(TreeNode::new(val)))),
                Some(np) => np.borrow_mut().insert(val),
            },
            false => match self.right.as_mut() {
                None => self.right = Some(Rc::new(RefCell::new(TreeNode::new(val)))),
                Some(np) => np.borrow_mut().insert(val),
            },
        }
    }
}


#[allow(dead_code)]
pub struct Solution;

use std::collections::BinaryHeap;

#[allow(dead_code)]
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        //Solution::by_heap(root, k)
        Solution::by_inorder(root, k)
    }

    pub fn by_inorder(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut res = 0;
        let mut k = k;
        Solution::inorder(root, &mut k, &mut res);
        res
    }

    fn inorder(np: Option<Rc<RefCell<TreeNode>>>, k:&mut i32, res: &mut i32) {
        if *k <= 0 { return; }

        if let Some(np) = np {
            Solution::inorder(np.borrow().left.clone(), k, res);
            *k -= 1;
            if *k == 0 {
                *res = np.borrow().val;
            }
            Solution::inorder(np.borrow().right.clone(), k, res);
        }
    }

    pub fn by_heap(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut h = BinaryHeap::new();
        Solution::iter(root, k, &mut h);
        
        *h.peek().unwrap()
    }

    fn iter(np: Option<Rc<RefCell<TreeNode>>>, k: i32, h: &mut BinaryHeap<i32>) {
        if let Some(np) = np {
            h.push(np.borrow().val);

            if h.len() > k as usize {
                h.pop();
            }

            Solution::iter(np.borrow().left.clone(), k, h);
            Solution::iter(np.borrow().right.clone(), k, h);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! build_tree {
        ( $root:expr, $( $rest:expr ),* ) => (
            {
                let mut np = TreeNode::new($root);
                build_tree!(np $($rest),*);
                np
            }
        );

        ( $root:tt $( $e:expr ),* ) => (
            $(
                $root.insert($e);
            )*
        );
    }

    #[test]
    pub fn test_by_heap() {
        {
            let root = build_tree!(3i32,);
            assert_eq!(Solution::kth_smallest(Some(Rc::new(RefCell::new(root))), 1), 3);
        }
        {
            let root = build_tree!(3i32, 1, 4, 2);

            eprintln!("{}", root);
            assert_eq!(Solution::kth_smallest(Some(Rc::new(RefCell::new(root))), 1), 1);
        }
        {
            let root = build_tree!(5i32, 3, 6, 2, 4, 1);
            assert_eq!(Solution::kth_smallest(Some(Rc::new(RefCell::new(root))), 3), 3);
        }
    }

}

