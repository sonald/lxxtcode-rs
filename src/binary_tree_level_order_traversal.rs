use super::utils::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let mut res = vec![];
        let mut q = VecDeque::new();
        let mut level = vec![];

        if root.is_none() { return res; }
        q.push_back(None);
        q.push_back(root);

        while let Some(np) = q.pop_front() {
            match np {
                Some(np) => {
                    level.push(np.borrow().val); 
                    if np.borrow().left.is_some() {
                        q.push_back(np.borrow().left.clone());
                    }
                    if np.borrow().right.is_some() {
                        q.push_back(np.borrow().right.clone());
                    }
                },
                _ => {
                    if level.len() > 0 { res.push(level); }
                    if q.len() == 0 { break }
                    q.push_back(None);
                    level = vec![];
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn method_test() {
        {
            assert_eq!(Solution::level_order(None), Vec::<Vec<i32>>::new());
        }
        {
            let root = TreeNode::new(21);
            assert_eq!(Solution::level_order(Some(Rc::new(RefCell::new(root)))), vec![vec![21]]);
        }
        {
            let mut root = TreeNode::new(1);
            root.insert_left(2);
            root.insert_right(2);
            root.left.as_mut().unwrap().borrow_mut().insert_left(3);
            root.left.as_mut().unwrap().borrow_mut().insert_right(4);
            root.right.as_mut().unwrap().borrow_mut().insert_left(4);
            root.right.as_mut().unwrap().borrow_mut().insert_right(3);

            let v = vec!{
                vec![1],
                vec![2, 2],
                vec![3,4,4,3],
            };
            assert_eq!(Solution::level_order(Some(Rc::new(RefCell::new(root)))), v);
        }
        {
            let mut root = TreeNode::new(3);
            root.insert_left(9);
            root.insert_right(20);
            root.right.as_mut().unwrap().borrow_mut().insert_left(15);
            root.right.as_mut().unwrap().borrow_mut().insert_right(7);
            let v = vec!{
                vec![3],
                vec![9, 20],
                vec![15, 7],
            };

            assert_eq!(Solution::level_order(Some(Rc::new(RefCell::new(root)))), v);
        }
    }
}
