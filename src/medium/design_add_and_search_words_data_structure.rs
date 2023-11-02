#![allow(dead_code)]

// For this problem, you need to create a Prefix Tree (Trie) that supports the following operations:
// 1. void addWord(word)
// 2. bool search(word)
// For this problem we already implement trie in another leetcode exercice, so we can reuse it.
// This problem need you to resolve first leetcode problem: https://leetcode.com/problems/implement-trie-prefix-tree/

use super::implement_trie_prefix_tree::Trie;

pub struct WordDictionary {
    trie: Trie,
}

impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary { trie: Trie::new() }
    }

    pub fn add_word(&mut self, word: String) {
        self.trie.insert(word);
    }

    pub fn search(&self, word: String) -> bool {
        fn helper(node: &Trie, word: &[u8]) -> bool {
            if word.is_empty() {
                return node.is_end;
            }
            match word[0] {
                b'.' => node.children.iter().any(|child| {
                    child
                        .as_ref()
                        .map_or(false, |child| helper(child, &word[1..]))
                }),
                ch => node.children[(ch - b'a') as usize]
                    .as_ref()
                    .map_or(false, |child| helper(child, &word[1..])),
            }
        }

        helper(&self.trie, word.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_dictionary() {
        let mut obj = WordDictionary::new();
        obj.add_word("bad".to_string());
        obj.add_word("dad".to_string());
        obj.add_word("mad".to_string());
        assert_eq!(obj.search("pad".to_string()), false);
        assert_eq!(obj.search("bad".to_string()), true);
        assert_eq!(obj.search(".ad".to_string()), true);
        assert_eq!(obj.search("b..".to_string()), true);
    }
}
