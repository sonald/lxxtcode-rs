use super::utils::*;

use std::cmp::Ordering;

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        //Self::by_pq(lists)
        Self::by_dnc(lists)
    }

    fn by_pq(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let mut root = None;
        let mut sp = &mut root;
        let mut heap = BinaryHeap::new();

        for n in lists {
            if n.is_some() {
                heap.push(Reverse(n.unwrap()));
            }
        }

        while let Some(np) = heap.pop() {
            *sp = Some(np.0);
            sp = &mut sp.as_mut().unwrap().next;

            if sp.is_some() {
                heap.push(Reverse(sp.take().unwrap()));
            }
        }

        root
    }

    //divide & conquer
    fn by_dnc(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {

        fn merge(mut lhs: Option<Box<ListNode>>, mut rhs: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut root = None;
            let mut sp = &mut root;
            while lhs.is_some() && rhs.is_some() {
                if lhs <= rhs {
                    *sp = lhs;
                    sp = &mut sp.as_mut().unwrap().next;
                    lhs = sp.take();
                } else {
                    *sp = rhs;
                    sp = &mut sp.as_mut().unwrap().next;
                    rhs = sp.take();
                }
            }

            if lhs.is_some() {
                *sp = lhs;
            }

            if rhs.is_some() {
                *sp = rhs;
            }

            root
        }

        fn divide(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
            if lists.len() <= 2 {
                return merge(lists.pop().unwrap_or(None), lists.pop().unwrap_or(None))
            }

            let rhs = lists.split_off(lists.len()/2);
            let l = divide(lists);
            let r = divide(rhs);

            merge(l, r)
        }

        divide(lists)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    //extern crate test;
    //use test::Bencher;
    use super::*;

    pub fn test_method(f: impl Fn(Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>>) {
        {
            let lists = vec![ ];
            let res = f(lists);
            let expect = None;
            list_check_equal(&res, &expect);
        }
        {
            let lists = vec!{
                build_list!(1,4,5),
                build_list!(1,3,4),
                build_list!(2,6),
                build_list!(2,3,4,7,9),
            };

            let res = f(lists);
            let expect = build_list!(1, 1, 2, 2, 3, 3, 4, 4, 4, 5, 6, 7, 9);
            list_check_equal(&res, &expect);
        }
        {
            let lists = vec!{
                build_list!(1,4,5),
                build_list!(1,3,4),
                build_list!(2,6),
            };

            let res = f(lists);
            let expect = build_list!(1, 1, 2, 3, 4, 4, 5, 6);
            list_check_equal(&res, &expect);
        }
    }

    #[test]
    pub fn test_by_dnc() {
        test_method(Solution::by_dnc);
    }

    #[test]
    pub fn test_by_pq() {
        test_method(Solution::by_pq);
    }
}


