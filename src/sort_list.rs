use super::utils::*;

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::by_qsort(head)
    }

    fn by_qsort(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        head
    }

    /// which is faster ? merge or merge2
    fn merge(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut lp = &mut head;

        loop {
            match (list1, list2) {
                (Some(mut l), Some(mut r)) => {
                    match l.val <= r.val {
                        true => {
                            list1 = l.next.take();
                            list2 = Some(r);
                            *lp = Some(l);
                        }

                        false => {
                            list1 = Some(l);
                            list2 = r.next.take();
                            *lp = Some(r);
                        }
                    }
                }

                (Some(mut l), None) => {
                    list1 = l.next.take();
                    list2 = None;
                    *lp = Some(l);
                }

                (None, Some(mut r)) => {
                    list2 = r.next.take();
                    list1 = None;
                    *lp = Some(r);
                },
                _ =>  break 
            }

            lp = &mut lp.as_mut().unwrap().next;
        }

        head
    }

    fn merge2(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut lp = &mut head;

        while let (true, true) = (list1.is_some(), list2.is_some()) {
            match list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                true => {
                    *lp = list1;
                    list1 = lp.as_mut().unwrap().next.take();
                }

                false => {
                    *lp = list2;
                    list2 = lp.as_mut().unwrap().next.take();
                }
            }
            lp = &mut lp.as_mut().unwrap().next;
        }

        if list1.is_some() {
            *lp = list1;
        }
        if list2.is_some() {
            *lp = list2;
        }

        head
    }

    fn by_mergesort(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        fn mergesort(mut list: Option<Box<ListNode>>, len: usize) -> Option<Box<ListNode>> {
            if len <= 1 {
                return list;
            }

            let np = Solution::list_get(&mut list, len/2-1);
            let list2 = np.as_mut().unwrap().next.take();

            let lhs = mergesort(list, len/2);
            let rhs = mergesort(list2, len - len/2);

            Solution::merge(lhs, rhs)
        }


        let len = Solution::list_len(&head);
        mergesort(head, len)
    }

    fn list_get(head: &mut Option<Box<ListNode>>, mut n: usize) -> &mut Option<Box<ListNode>> {
        let mut lp = head;
        loop {
            if n == 0 {
                break
            }

            if let Some(ref mut inner) = lp {
                lp = &mut inner.next;
            } else {
                break
            }
            n -= 1;
        }

        lp
    }

    fn list_len(head: &Option<Box<ListNode>>) -> usize {
        let mut sz = 0;
        let mut lp = head;
        while let Some(ref inner) = lp {
            sz += 1;
            lp = &inner.next;
        }

        sz
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    //extern crate test;
    //use test::Bencher;
    use super::*;

    pub fn test_method<F>(f: F) where F: Fn(Option<Box<ListNode>>)->Option<Box<ListNode>> {
        {
            let list = None;
            let res = f(list);
            let expect = None;

            list_check_equal(&res, &expect);
        }
        {
            let list = build_list!(4,);
            let res = f(list);
            let expect = build_list!(4,);

            list_check_equal(&res, &expect);
        }
        {
            let list = build_list!(4,2);
            let res = f(list);
            let expect = build_list!(2,4);

            list_check_equal(&res, &expect);
        }
        {
            let list = build_list!(4,2,1);
            let res = f(list);
            let expect = build_list!(1,2,4);

            list_check_equal(&res, &expect);
        }
        {
            let list = build_list!(4,2,1,3);
            let res = f(list);
            let expect = build_list!(1,2,3,4);

            list_check_equal(&res, &expect);
        }
    }

    #[test]
    pub fn test_helpers() {
        {
            let mut list = None;
            assert_eq!(Solution::list_len(&list), 0);
            assert_eq!(Solution::list_get(&mut list, 6).is_none(), true);
            assert_eq!(Solution::list_get(&mut list, 0).is_none(), true);
        }
        {
            let mut list = build_list!(4,);
            assert_eq!(Solution::list_len(&list), 1);
            assert_eq!(Solution::list_get(&mut list, 0).as_ref().unwrap().val, 4);
            assert_eq!(Solution::list_get(&mut list, 6).is_none(), true);
        }
        {
            let mut list = build_list!(4,2,1,3,7);
            assert_eq!(Solution::list_len(&list), 5);
            assert_eq!(Solution::list_get(&mut list, 0).as_ref().unwrap().val, 4);
            assert_eq!(Solution::list_get(&mut list, 1).as_ref().unwrap().val, 2);
            assert_eq!(Solution::list_get(&mut list, 2).as_ref().unwrap().val, 1);
            assert_eq!(Solution::list_get(&mut list, 3).as_ref().unwrap().val, 3);
            assert_eq!(Solution::list_get(&mut list, 6).is_none(), true);
        }
        {
            let mut list = build_list!(4,2,1,3);
            assert_eq!(Solution::list_len(&list), 4);
            assert_eq!(Solution::list_get(&mut list, 0).as_ref().unwrap().val, 4);
            assert_eq!(Solution::list_get(&mut list, 1).as_ref().unwrap().val, 2);
            assert_eq!(Solution::list_get(&mut list, 2).as_ref().unwrap().val, 1);
            assert_eq!(Solution::list_get(&mut list, 3).as_ref().unwrap().val, 3);
            assert_eq!(Solution::list_get(&mut list, 4).is_none(), true);
        }
        {
            let mut list = build_list!(1,2,4);
            let mut list2 = build_list!(1,3,5,7);
            let mut res = Solution::merge(list, list2);
            assert_eq!(Solution::list_len(&res), 7);
            assert_eq!(Solution::list_get(&mut res, 0).as_ref().unwrap().val, 1);
            assert_eq!(Solution::list_get(&mut res, 1).as_ref().unwrap().val, 1);
            assert_eq!(Solution::list_get(&mut res, 2).as_ref().unwrap().val, 2);
            assert_eq!(Solution::list_get(&mut res, 3).as_ref().unwrap().val, 3);
            assert_eq!(Solution::list_get(&mut res, 4).as_ref().unwrap().val, 4);
            assert_eq!(Solution::list_get(&mut res, 5).as_ref().unwrap().val, 5);
            assert_eq!(Solution::list_get(&mut res, 6).as_ref().unwrap().val, 7);
            assert_eq!(Solution::list_get(&mut res, 7).is_none(), true);
        }
    }

    #[test]
    pub fn test_by_qsort() {
        test_method(Solution::by_mergesort);
    }
}

