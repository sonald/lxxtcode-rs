use super::utils::*;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preorder:&[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match inorder.len() > 0 {
                true => {
                    let x = preorder[0];
                    let i = inorder.iter().position(|&y| y == x).unwrap();
                    let mut root = TreeNode::new(x);
                    root.left = build(&preorder[1..], &inorder[0..i]);
                    root.right = build(&preorder[i+1..], &inorder[i+1..]);
                    Some(Rc::new(RefCell::new(root)))
                }

                _ => None
            }
        }

        build(&preorder, &inorder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_test() {
        let root = Solution::build_tree(vec![3,9,20,15,7], vec![9,3,15,20,7]);
        eprintln!("{}", root.unwrap().borrow())
    }
}
