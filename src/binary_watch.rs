use std::vec::Vec;

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        Solution::by_counting(num)
    }

    fn by_counting(num: i32) ->Vec<String> {
        use std::collections::HashMap;

        let mut res = vec![];
        let mut h = HashMap::new();

        for i in 0..60.min(2i32.pow(6)) {
            h.entry(i.count_ones() as i32).or_insert(vec![]).push(i);
        }

        for n in 0..=num {
            let m = num - n;
            if n > 4 || m > 6 { continue; }

            let secs = h.get(&m).unwrap();
            h.get(&n).unwrap().iter().for_each(|&c| {
                if c <= 11 {
                    for &s in secs {
                        res.push(format!("{}:{:02}", c, s));
                    }
                }
            });
        }

        res
    }

    fn by_bits(num: i32) -> Vec<String> {
        let mut res = vec![];

        for h in 0..12 {
            for m in 0..60 {
                let v = h * 64 + m as i32;
                if v.count_ones() == num as u32 {
                    res.push(format!("{}:{:02}", h, m));
                }
            }
        }

        res
    }

    //TODO:
    fn by_backtracking(num: i32) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn test_method<F>(f: F) where F: Fn(i32)->Vec<String> {
        {
            let expect = vec!["0:00"];
            let res = f(0);
            assert_eq!(res, expect);
        }
        {
            let expect = ["1:00", "2:00", "4:00", "8:00", "0:01", "0:02", "0:04", "0:08", "0:16", "0:32"]
                .into_iter().map(|v| v.to_string()).collect::<HashSet<_>>();
            let res = f(1).into_iter().collect::<HashSet<_>>();
            assert_eq!(res, expect);
        }
        {
            let expect = ["0:03","0:05","0:06","0:09","0:10","0:12","0:17","0:18","0:20","0:24","0:33","0:34","0:36","0:40","0:48","1:01","1:02","1:04","1:08","1:16","1:32","2:01","2:02","2:04","2:08","2:16","2:32","3:00","4:01","4:02","4:04","4:08","4:16","4:32","5:00","6:00","8:01","8:02","8:04","8:08","8:16","8:32","9:00","10:00"]
                .into_iter().map(|v| v.to_string()).collect::<HashSet<_>>();
            let res = f(2).into_iter().collect::<HashSet<_>>();
            assert_eq!(res, expect);
        }
        
        //println!("{:?}", f(7));
    }

    #[test]
    pub fn test_by_bits() {
        test_method(Solution::by_bits);
    }

    #[test]
    pub fn test_by_counting() {
        test_method(Solution::by_counting);
    }

}


