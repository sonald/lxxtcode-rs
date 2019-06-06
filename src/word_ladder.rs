use std::vec::Vec;

#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        Solution::by_iter(begin_word, end_word, word_list)
    }

    fn distance(s1: &str, s2: &str) -> usize {
        s1.chars().zip(s2.chars()).filter(|(c1, c2)| c1 != c2).count()
    }

    fn by_iter(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        use std::collections::HashSet;

        let mut words = word_list.iter().map(|s| s.as_str()).collect::<HashSet<&str>>();
        if !words.remove(end_word.as_str()) {
            return 0
        }

        let mut sp = vec![end_word.as_str()];
        let mut level = 0;

        while sp.len() > 0 {
            level += 1;
            let mut newsp = vec![];
            for t in &sp {
                let mut newwords = HashSet::<&str>::new();

                if Solution::distance(&begin_word, t) == 1 {
                    return level+1;
                }

                for s in &words {
                    if Solution::distance(s, t) == 1 {
                        newsp.push(*s);
                    } else {
                        newwords.insert(s);
                    }
                }


                std::mem::swap(&mut words, &mut newwords);
            }
            std::mem::swap(&mut sp, &mut newsp);
        }
        
        0
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    fn test_method<F>(f: F) where F: Fn(String, String, Vec<String>) -> i32 {
        {
            let list = ["hot","dot","dog","lot","log"].iter().map(|s| s.to_string()).collect();
            assert_eq!(f("hit".to_string(), "cog".to_string(), list), 0);
        }
        {
            let list = ["hot","dot","dog","lot","log","cog"].iter().map(|s| s.to_string()).collect();
            assert_eq!(f("hit".to_string(), "cog".to_string(), list), 5);
        }
        {
            let list = ["hot","dot","dog","lot","log","cog"].iter().map(|s| s.to_string()).collect();
            assert_eq!(f("dog".to_string(), "cog".to_string(), list), 2);
        }
        {
            let list = ["hot","dot","dog","lot","log","cog"].iter().map(|s| s.to_string()).collect();
            assert_eq!(f("lot".to_string(), "cog".to_string(), list), 3);
        }
        {
            let list = ["hot","dog"].iter().map(|s| s.to_string()).collect();
            assert_eq!(f("hot".to_string(), "dog".to_string(), list), 0);
        }
        {
            let list = include!("./word_ladder.txt").iter().map(|s| s.to_string()).collect();
            assert_eq!(f("sand".to_string(), "acne".to_string(), list), 11);
        }

    }


    #[test]
    pub fn test_basic() {
        test_method(Solution::by_iter);
    }


    #[test]
    pub fn test_iter() {
        test_method(Solution::by_iter);
    }

    #[bench]
    pub fn bench_basic(b: &mut Bencher) {
        let list: Vec<String> = include!("./word_ladder.txt").iter().map(|s| s.to_string()).collect();
        b.iter(|| {
            Solution::by_iter("sand".to_string(), "acne".to_string(), list.clone());
        })
    }
}



