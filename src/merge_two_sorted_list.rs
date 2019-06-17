use super::utils::*;

#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut res = None;
        let mut sp = &mut res;

        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val >= l2.as_ref().unwrap().val {
                *sp = l1;
                sp = &mut sp.as_mut().unwrap().next;
                l1 = sp.take();
            } else {
                *sp = l2;
                sp = &mut sp.as_mut().unwrap().next;
                l2 = sp.take();
            }
        }

        if l1.is_some() {
            *sp = l1;
        }

        if l2.is_some() {
            *sp = l2;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_merge() {}
}
