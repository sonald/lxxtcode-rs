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


#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        Solution::sol2(root)
    }

    pub fn sol2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let mut res = vec![];
        let mut q = VecDeque::new();

        if root.is_none() { return res; }
        q.push_back(root);
        q.push_back(None);
        res.push(vec![]);

        loop {
            match q.pop_front() {
                Some(Some(np)) => {
                    if let Some(last) = res.last_mut() {
                        last.push(np.borrow().val);
                        [&np.borrow().left, &np.borrow().right].iter()
                            .filter(|x| x.is_some()).for_each(|&x| q.push_back(x.clone()));
                    }
                },
                Some(None) => {
                    if q.len() == 0 {
                        break
                    }

                    q.push_back(None);
                    res.push(vec![]);
                },
                _ => {unreachable!("")}
            } 
        }

        let mut reversed = false;
        for v in res.iter_mut() {
            if reversed {
                v.reverse();
            }
            reversed = !reversed;
        }

        res
    }

    pub fn sol1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let mut reversed = true;
        let mut res = vec![vec![]];
        let mut q = VecDeque::new();
        let mut p = VecDeque::new();

        if root.is_none() { return vec![]; }
        q.push_back(root);
        q.push_back(None);

        loop {
            match q.pop_front() {
                Some(Some(np)) => {
                    if let Some(last) = res.last_mut() {
                        last.push(np.borrow().val);
                        p.push_front(Some(np));
                    }

                },
                Some(None) => {
                    if p.len() == 0 {
                        break
                    }

                    while let Some(np) = p.pop_front() {
                        let np = np.unwrap();
                        if reversed {
                            [&np.borrow().right, &np.borrow().left].iter()
                                .filter(|x| x.is_some()).for_each(|&x| q.push_back(x.clone()));
                        } else {
                            [&np.borrow().right, &np.borrow().left].iter().rev()
                                .filter(|x| x.is_some()).for_each(|&x| q.push_back(x.clone()));
                        }
                    }

                    if q.len() == 0 {
                        break
                    }
                    q.push_back(None);
                    reversed = !reversed;
                    res.push(vec![]);
                },
                _ => {unreachable!("")}
            } 
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
            assert_eq!(Solution::zigzag_level_order(None), Vec::new() as Vec<Vec<_>>);
        }
        {
            let mut root = TreeNode::new(3);
            root.insert_left(9);
            root.insert_right(20);
            root.right.as_mut().unwrap().borrow_mut().insert_left(15);
            root.right.as_mut().unwrap().borrow_mut().insert_right(7);

            eprintln!("{}", root);
            let res = vec![vec![3], vec![20, 9], vec![15, 7]];
            assert_eq!(Solution::zigzag_level_order(Some(Rc::new(RefCell::new(root)))), res);
        }
        {
            let mut root = TreeNode::new(1);
            root.insert_left(2);
            root.insert_right(3);
            root.left.as_mut().unwrap().borrow_mut().insert_left(4);
            root.right.as_mut().unwrap().borrow_mut().insert_right(5);

            eprintln!("{}", root);
            let res = vec![vec![1], vec![3, 2], vec![4, 5]];
            assert_eq!(Solution::zigzag_level_order(Some(Rc::new(RefCell::new(root)))), res);
        }
    }

}


