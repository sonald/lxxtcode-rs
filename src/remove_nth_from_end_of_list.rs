use super::utils::*;

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut k = n;
        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head;

        let mut fast = &dummy;
        while k > 0 {
            fast = &fast.as_ref().unwrap().next;
            k -= 1;
        }

        while let Some(inner) = fast {
            fast = &inner.next;
            k += 1;
        }
        k -= 1;

        //NOTE: In c++, I can move slow poiner along with fast pointer,
        //but rust prevent me from doing that.
        let mut slow = &mut dummy;
        while k > 0 {
            slow = &mut slow.as_mut().unwrap().next;
            k -= 1;
        }
        
        let next = slow.as_mut().unwrap().next.as_mut().unwrap().next.take();
        slow.as_mut().unwrap().next = next;

        dummy.unwrap().next
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    //extern crate test;
    //use test::Bencher;
    use super::*;

    pub fn test_method<F>(f: F) where F: Fn(Option<Box<ListNode>>, i32) -> Option<Box<ListNode>> {
        {
            let list = build_list!(4,);
            let res = f(list, 1);
            let expect = None;

            list_check_equal(&res, &expect);
        }
        {
            let list = build_list!(4,2,1,3);
            let res = f(list, 1);
            let expect = build_list!(4,2,1);

            list_check_equal(&res, &expect);
        }
        {
            let list = build_list!(4,2,1,3);
            let res = f(list, 2);
            let expect = build_list!(4,2,3);

            list_check_equal(&res, &expect);
        }
        {
            let list = build_list!(4,2,1,3);
            let res = f(list, 3);
            let expect = build_list!(4,1,3);

            list_check_equal(&res, &expect);
        }
        {
            let list = build_list!(4,2,1,3);
            let res = f(list, 4);
            let expect = build_list!(2,1,3);

            list_check_equal(&res, &expect);
        }
    }

    #[test]
    pub fn test_by_qsort() {
        test_method(Solution::remove_nth_from_end);
    }
}


