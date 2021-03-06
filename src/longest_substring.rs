use std::collections::{HashSet, HashMap};

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut h = HashSet::new();
        let bytes = s.as_bytes();
        let mut max = 0;
        let mut i = 0;
        let mut j = 0;

        while i < bytes.len() && j < bytes.len() {
            match h.get(&bytes[j]) {
                None => {
                    h.insert(bytes[j]);
                    j += 1;
                    max = max.max(j-i);
                },
                _ => {
                    h.remove(&bytes[i]);
                    i += 1;
                }
            }
        }

        max as i32
    }

    pub fn length_of_longest_substring2(s: String) -> i32 {
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

    pub fn length_of_longest_substring3(s: String) -> i32 {
        let mut h = [None; 256];
        let bytes = s.as_bytes();
        let mut max = 0;
        let mut start = 0;

        for (j, &ch) in bytes.iter().enumerate() {
            if let Some(last) = h[ch as usize] {
                start = start.max(last+1);
            }
            h[ch as usize] = Some(j);
            max = max.max(j-start+1);
        }

        max as i32
    }

    pub fn length_of_longest_substring4(s: String) -> i32 {
        let mut h :HashMap<u8, usize> = HashMap::new();
        let bytes = s.as_bytes();
        let mut max = 0;
        let mut i = 0;
        let mut j = 0;

        while i < bytes.len() && j < bytes.len() {
            if let Some(&last) = h.get(&bytes[j]) {
                i = (last+1).max(i);
            }
            h.insert(bytes[j], j);
            max = max.max(j-i+1);
            j += 1;
        }

        max as i32
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
        assert_eq!(Solution::length_of_longest_substring3("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring3("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring3("pwwkew".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring3(" ".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring3("dvdf".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring3("abba".to_string()), 2);
        assert_eq!(Solution::length_of_longest_substring3("tmmzuxt".to_string()), 5);
        

        let s = gen_random_string();
        assert_eq!(Solution::length_of_longest_substring3(s.clone()),
            Solution::length_of_longest_substring(s));
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
            Solution::length_of_longest_substring3(s);
        })
    }

    #[bench]
    fn bench_longest_substring3(b: &mut Bencher) {
        b.iter(|| {
            let s = gen_random_string();
            Solution2::length_of_longest_substring(s);
        })
    }
}

