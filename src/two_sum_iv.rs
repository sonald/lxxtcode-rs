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

use std::collections::HashSet;

pub struct Solution2;
#[allow(dead_code)]
impl Solution2 {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut h = HashSet::new();
        Self::find2(root.clone(), k, &mut h)
    }

    pub fn find2(np: Option<Rc<RefCell<TreeNode>>>, k: i32, h: &mut HashSet<i32>) -> bool {
        if let Some(np) = np {
            match h.contains(&np.borrow().val) {
                true => return true,
                _ => {
                    let rem = k - np.borrow().val;
                    h.insert(rem);

                    Self::find2(np.borrow().left.clone(), k, h)
                        || Self::find2(np.borrow().right.clone(), k, h)
                }
            }
        } else {
            false
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
    pub fn test_twosum_iv2() {
        {
            let root = build_tree!(5i32, 3, 2, 4, 6, 7);

            eprintln!("{}", root);
            assert_eq!(
                Solution2::find_target(Some(Rc::new(RefCell::new(root))), 9),
                true
            );
        }
        {
            let root = build_tree!(5i32, 3, 2, 4, 6, 7);
            assert_eq!(
                Solution2::find_target(Some(Rc::new(RefCell::new(root))), 11),
                true
            );
        }
        {
            let root = build_tree!(5i32, 3, 2, 4, 6, 7);
            assert_eq!(
                Solution2::find_target(Some(Rc::new(RefCell::new(root))), 13),
                true
            );
        }
    }

}
