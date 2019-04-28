// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = None;
        let mut parent: &mut Option<Box<ListNode>> = &mut root;

        let mut carry = 0;

        let mut lhs = l1.clone();
        let mut rhs = l2.clone();
        loop {
            if lhs.is_none() && rhs.is_none() {
                break
            }

            let (mut v1, mut v2) = (0, 0);
            if let Some(ref lhs) = lhs {
                v1 = lhs.val;
            }
            if let Some(ref rhs) = rhs {
                v2 = rhs.val;
            }

            let v = (v1+v2+carry) % 10;
            carry = (v1+v2+carry) / 10;

            let np = Some(Box::new(ListNode::new(v)));
            if let Some(ref mut inner) = parent {
                inner.next = np;
                parent = &mut inner.next;
            } else {
                root = np;
                parent = &mut root;
            }

            lhs = lhs.and_then(|l| l.next);
            rhs = rhs.and_then(|l| l.next);
        }

        if carry > 0 {
            let np = Some(Box::new(ListNode::new(carry)));
            if let Some(ref mut inner) = parent {
                inner.next = np;
            }
        }
        root
    }

    // this is faster than the above one, exactly why ?
    pub fn add_two_numbers2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = Some(Box::new(ListNode::new(0)));
        let mut parent: &mut Option<Box<ListNode>> = &mut root;

        let mut carry = 0;

        let mut lhs = &l1;
        let mut rhs = &l2;
        loop {
            let v;
            match (lhs, rhs) {
                (Some(l), Some(r)) => {
                    v = (l.val + r.val + carry) % 10;
                    carry = (l.val + r.val + carry) / 10;

                    lhs = &l.next;
                    rhs = &r.next;
                },
                (Some(l), None) => {
                    v = (l.val + carry) % 10;
                    carry = (l.val + carry) / 10;

                    lhs = &l.next;
                },
                (None, Some(r)) => {
                    v = (r.val + carry) % 10;
                    carry = (r.val + carry) / 10;
                    rhs = &r.next;
                },
                _ => {
                    if carry > 0 {
                        let np = Some(Box::new(ListNode::new(carry)));
                        if let Some(ref mut inner) = parent {
                            inner.next = np;
                        }
                    }
                    break;
                }
            }

            let np = Some(Box::new(ListNode::new(v)));
            if let Some(ref mut inner) = parent {
                inner.next = np;
                parent = &mut inner.next;
            }
        }

        root.unwrap().next
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    extern crate test;
    use test::Bencher;
    use super::*;

    macro_rules! build_list {
        ($first:expr, $($rest:expr),*) => (
            {
                let mut root = Some(Box::new(ListNode::new($first)));
                let mut parent = &mut root;
                $(

                    let np = Some(Box::new(ListNode::new($rest)));
                    if let Some(ref mut inner) = parent {
                        inner.next = np;
                        parent = &mut inner.next;
                    }
                )*

                root
            }
        )
    }

    macro_rules! build_rand_list {
        () => (
            {
                let mut root = Some(Box::new(ListNode::new(0)));
                let mut parent = &mut root;

                for i in 1..10000 {
                    let np = Some(Box::new(ListNode::new(i%10)));
                    if let Some(ref mut inner) = parent {
                        inner.next = np;
                        parent = &mut inner.next;
                    }
                }
                root
            }
        )
    }

    fn print_list(l: &Option<Box<ListNode>>) {
        if let Some(ref l) = l {
            print!("{} -> ", l.val);
            print_list(&l.next);
        }
    }

    fn check_equal(l: &Option<Box<ListNode>>, r: &Option<Box<ListNode>>) {
        match (l, r) {
            (Some(l), Some(r)) => {
                assert_eq!(l.val, r.val);
                check_equal(&l.next, &r.next);
            },
            (None, None) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    pub fn test_add_two_nums() {
        //Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
        //Output: 7 -> 0 -> 8
        //Explanation: 342 + 465 = 807.

        let lhs = build_list!(2,4,3);
        let rhs = build_list!(5,6,4);
        let res = Solution::add_two_numbers(lhs, rhs);
        let expect = build_list!(7, 0, 8);

        check_equal(&res, &expect);
    }

    #[test]
    pub fn test_add_two_nums2() {
        let lhs = build_list!(7,2,4,1);
        let rhs = build_list!(9,5,8);
        let res = Solution::add_two_numbers(lhs, rhs);
        let expect = build_list!(6, 8, 2, 2);

        check_equal(&res, &expect);
    }

    #[test]
    pub fn test_add_two_nums3() {
        let lhs = build_list!(5,);
        let rhs = build_list!(5,);
        let res = Solution::add_two_numbers(lhs, rhs);
        //print_list(&res);
        let expect = build_list!(0, 1);

        check_equal(&res, &expect);
    }


    #[test]
    pub fn test_add_two_nums4() {
        let lhs = build_list!(7,2,4,1);
        let rhs = build_list!(9,5,8);
        let res = Solution::add_two_numbers2(lhs, rhs);
        let expect = build_list!(6, 8, 2, 2);

        check_equal(&res, &expect);
    }

    #[test]
    pub fn test_add_two_nums5() {
        let lhs = build_list!(5,);
        let rhs = build_list!(5,);
        let res = Solution::add_two_numbers2(lhs, rhs);
        //print_list(&res);
        let expect = build_list!(0, 1);

        check_equal(&res, &expect);
    }

    #[bench]
    pub fn bench_ver1(b: &mut Bencher) {
        b.iter(|| {
            let lhs = build_rand_list!();
            let rhs = build_rand_list!();
            Solution::add_two_numbers(lhs, rhs);
        });
    }

    #[bench]
    pub fn bench_ver2(b: &mut Bencher) {
        b.iter(|| {
            let lhs = build_rand_list!();
            let rhs = build_rand_list!();
            Solution::add_two_numbers2(lhs, rhs);
        });
    }
}
