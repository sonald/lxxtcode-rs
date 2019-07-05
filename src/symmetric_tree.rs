use super::utils::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::by_iterative(root)
    }

    pub fn by_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn recursive(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (l, r) {
                (Some(l1), Some(r1)) => {
                    l1.borrow().val == r1.borrow().val
                        && recursive(l1.borrow().left.clone(), r1.borrow().right.clone())
                        && recursive(l1.borrow().right.clone(), r1.borrow().left.clone())
                }
                (None, None) => true,
                _ => false,
            }
        }

        root.map_or(true, |np| {
            recursive(np.borrow().left.clone(), np.borrow().right.clone())
        })
    }

    pub fn by_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(np) = root {
            let mut sp1 = vec![np.borrow().left.clone()];
            let mut sp2 = vec![np.borrow().right.clone()];

            loop {
                match (sp1.pop(), sp2.pop()) {
                    (Some(Some(l)), Some(Some(r))) => {
                        if l.borrow().val != r.borrow().val {
                            return false
                        } else {
                            sp1.push(l.borrow().left.clone());
                            sp2.push(r.borrow().right.clone());

                            sp1.push(l.borrow().right.clone());
                            sp2.push(r.borrow().left.clone());
                        }
                    }
                    (Some(None), Some(None)) => continue,
                    (None, None) => break,
                    _ => return false
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn method_test(f: impl Fn(Option<Rc<RefCell<TreeNode>>>) -> bool) {
        {
            let root = TreeNode::new(2147483647);
            assert_eq!(f(Some(Rc::new(RefCell::new(root)))), true);
        }
        {
            let mut root = TreeNode::new(1);
            root.insert_left(2);
            root.insert_right(2);
            root.left.as_mut().unwrap().borrow_mut().insert_left(3);
            root.left.as_mut().unwrap().borrow_mut().insert_right(4);
            root.right.as_mut().unwrap().borrow_mut().insert_left(4);
            root.right.as_mut().unwrap().borrow_mut().insert_right(3);

            assert_eq!(f(Some(Rc::new(RefCell::new(root)))), true);
        }
        {
            let mut root = TreeNode::new(1);
            root.insert_left(2);
            root.insert_right(2);
            root.left.as_mut().unwrap().borrow_mut().insert_left(3);
            root.left.as_mut().unwrap().borrow_mut().insert_right(4);
            root.right.as_mut().unwrap().borrow_mut().insert_left(3);
            root.right.as_mut().unwrap().borrow_mut().insert_right(4);

            assert_eq!(f(Some(Rc::new(RefCell::new(root)))), false);
        }
        {
            let mut root = TreeNode::new(1);
            root.insert_left(2);
            root.insert_right(2);
            root.left.as_mut().unwrap().borrow_mut().insert_right(4);
            root.right.as_mut().unwrap().borrow_mut().insert_right(4);

            assert_eq!(f(Some(Rc::new(RefCell::new(root)))), false);
        }
    }

    #[test]
    fn sym_test() {
        method_test(Solution::by_iterative);
        method_test(Solution::by_recursive);
    }
}
