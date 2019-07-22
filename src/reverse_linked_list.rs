use crate::utils::*;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }

    pub fn by_iter(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = None;
        while head.is_some() {
            let mut np = head;
            head = np.as_mut().unwrap().next.take();
            np.as_mut().unwrap().next = root;
            root = np;
        }

        root
    }

    pub fn by_recurse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn reverse(np: Option<Box<ListNode>>, root: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
            if let Some(mut np) = np {
                return if np.next.is_some() {
                    let tail = reverse(np.next.take(), root);
                    *tail = Some(np);
                    &mut tail.as_mut().unwrap().next
                } else {
                    *root = Some(np);
                    &mut root.as_mut().unwrap().next
                };
            }

            root
        }

        let mut root = None;
        reverse(head, &mut root);
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_list_test() {
        let l = build_list!(1,2,3,4,5);
        let l = Solution::by_recurse(l);

        let l2 = build_list!(1,2,3,4,5);
        let l2 = Solution::by_iter(l2);
        //print_list(&l);
        let res = build_list!(5,4,3,2,1);
        list_check_equal(&l, &res);
        list_check_equal(&l2, &res);

        list_check_equal(&None, &None);
    }
}
