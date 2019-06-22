pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        Self::by_map(strs)
    }
    
    // another way: compute hash by generate unique integer for every
    // group using primes

    pub fn by_map(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut groups = HashMap::new();

        for s in strs {
            let mut k = s.clone();
            unsafe { k.as_bytes_mut().sort(); }
            groups.entry(k).or_insert(vec![]).push(s);
        }

        groups.drain().map(|(_, v)| v).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurse() {
        test_method(Solution::by_map);
    }

    
    fn test_method(f: impl Fn(Vec<String>) -> Vec<Vec<String>>) {
        let input = ["eat", "tea", "tan", "ate", "nat", "bat"].iter().map(|v|v.to_string()).collect::<Vec<_>>();
        let mut expect = vec![ 
            vec!["ate","eat","tea"],
            vec!["nat","tan"],
            vec!["bat"] ];
        expect.iter_mut().for_each(|v| v.sort());
        expect.sort();
        let mut res = f(input);
        res.iter_mut().for_each(|v| v.sort());
        res.sort();

        assert_eq!(res, expect);
    }
}

