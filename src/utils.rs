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

    // insert as BST
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

    // insert in arbitrary order
    pub fn insert_right(&mut self, val: i32) {
        if self.right.is_none() {
            self.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
    }

    pub fn insert_left(&mut self, val: i32) {
        if self.left.is_none() {
            self.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
    }
}

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

