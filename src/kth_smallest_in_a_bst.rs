use std::cell::RefCell;
use std::rc::Rc;
use super::utils::*;

#[allow(dead_code)]
pub struct Solution;

use std::collections::BinaryHeap;

#[allow(dead_code)]
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        //Solution::by_heap(root, k)
        //Solution::by_inorder(root, k)
        Solution::by_inorder_iterative(root, k)
    }

    pub fn by_inorder_iterative(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut sp = vec![];
        let mut k = k;

        let mut np = root;
        loop {
            while let Some(n) = np {
                sp.push(Some(n.clone()));
                np = n.borrow().left.clone();
            }

            if let Some(n) = sp.pop() {
                let n = n.unwrap();
                k -= 1;
                if k == 0 {
                    return n.borrow().val;
                }

                np = n.borrow().right.clone();
            } else {
                break
            }
        }

        0
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

