use std::collections::{HashSet, HashMap};

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut h = HashMap::new();
        let bytes = s.as_bytes();
        let mut max = 0;
        let mut sz = 0;

        for (i, ch) in bytes.iter().enumerate() {
            match h.get(ch) {
                None => {
                    sz += 1;
                    h.insert(*ch, i);
                },
                Some(&last) => {
                    max = max.max(sz);
                    ((i-sz)..=last).for_each(|j| { h.remove(&bytes[j]); });
                    h.insert(*ch, i);
                    sz = i - last;
                }
            }
        }
        
        max.max(sz) as i32
    }

    pub fn length_of_longest_substring2(s: String) -> i32 {
        let mut h = HashMap::new();
        let bytes = s.as_bytes();
        let mut max = 0;
        let mut sz = 0;
        let mut i = 0;
        while i < bytes.len() {
            match h.get(&bytes[i]) {
                None => {
                    sz += 1;
                    h.insert(bytes[i], i);
                },
                Some(&last) => {
                    max = max.max(sz);
                    h.clear();
                    ((last+1)..=i).for_each(|j| { h.insert(bytes[j], j); });
                    sz = (i - last) as i32;
                }
            }
            i += 1;
        }
        
        max.max(sz)
    }
}

#[allow(dead_code)]
pub struct Solution2;

#[allow(dead_code)]
impl Solution2 {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut max = 0;
        for i in 0..bytes.len() {
            let k = Solution2::find_longest_from(i, bytes);
            max = max.max(k);
        }
        
        max
    }

    fn find_longest_from(i: usize, bytes: &[u8]) -> i32 {
        let mut sz = 0;
        let mut m = HashSet::new();

        for k in i..bytes.len() {
            if m.contains(&bytes[k]) {
                break
            }

            sz += 1;
            m.insert(bytes[k]);
        }

        sz
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    extern crate test;
    use test::*;
    use super::*;

    fn gen_random_string() -> String {
        use rand::random;
        (1..1000).map(|_| char::from(random::<u8>())).collect()
    }

    #[test]
    fn test_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);

        let s = gen_random_string();
        assert_eq!(Solution::length_of_longest_substring(s.clone()),
            Solution2::length_of_longest_substring(s));
    }

    #[test]
    fn test_longest_substring2() {
        assert_eq!(Solution2::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution2::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution2::length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(Solution2::length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(Solution2::length_of_longest_substring("dvdf".to_string()), 3);
        assert_eq!(Solution2::length_of_longest_substring("abba".to_string()), 2);
    }

    #[bench]
    fn bench_longest_substring(b: &mut Bencher) {
        b.iter(|| {
            let s = gen_random_string();
            Solution::length_of_longest_substring(s);
        })
    }

    #[bench]
    fn bench_longest_substring2(b: &mut Bencher) {
        b.iter(|| {
            let s = gen_random_string();
            Solution2::length_of_longest_substring(s);
        })
    }
}

