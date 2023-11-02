#![allow(dead_code)]

pub struct Trie {
    pub is_end: bool,
    pub children: [Option<Box<Trie>>; 26],
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            is_end: false,
            children: Default::default(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = self;
        for ch in word.chars().map(|ch| (ch as u8 - 'a' as u8) as usize) {
            node = node.children[ch].get_or_insert(Box::new(Trie::new()));
        }
        node.is_end = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut node = self;
        for ch in word.chars().map(|ch| (ch as u8 - 'a' as u8) as usize) {
            match node.children[ch].as_ref() {
                Some(next_node) => node = next_node,
                None => return false,
            }
        }
        node.is_end
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;
        for ch in prefix.chars().map(|ch| (ch as u8 - 'a' as u8) as usize) {
            match node.children[ch].as_ref() {
                Some(next_node) => node = next_node,
                None => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple".to_string()), true);
        assert_eq!(trie.search("app".to_string()), false);
        assert_eq!(trie.starts_with("app".to_string()), true);
        trie.insert("app".to_string());
        assert_eq!(trie.search("app".to_string()), true);
    }
}
