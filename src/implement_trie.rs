#[derive(Default)]
struct Trie {
    tail: bool,
    children: [Option<Box<Trie>>; 26]
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }
    
    fn insert(&mut self, word: String) {
        let mut np = self;
        for i in word.chars().map(|ch| u32::from(ch) as usize - 97) {
            np = np.children[i].get_or_insert(Box::new(Trie::new()));
        }
        np.tail = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut np = self;
        for i in word.chars().map(|ch| u32::from(ch) as usize - 97) {
            match np.children[i].as_ref() {
                Some(child) => np = child,
                _ => return false,
            }
        }

        np.tail 
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut np = self;
        for i in prefix.chars().map(|ch| u32::from(ch) as usize - 97) {
            match np.children[i].as_ref() {
                Some(child) => np = child,
                _ => return false
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie_test() {
        let mut trie = Trie::new();

        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple".to_string()), true);
        assert_eq!(trie.search("app".to_string()), false);
        assert_eq!(trie.starts_with("app".to_string()), true);

        trie.insert("app".to_string());   
        assert_eq!(trie.search("app".to_string()), true);    

        trie.insert("bpple".to_string());   
        assert_eq!(trie.search("bpple".to_string()), true);
        
    }
}
