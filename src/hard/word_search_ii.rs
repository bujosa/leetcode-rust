#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = self;

        for c in word.chars() {
            node = node.children.entry(c).or_insert(TrieNode::new());
        }
        node.is_word = true;
    }
}

fn find_words(board: Vec<Vec<char>>, words: Vec<&str>) -> Vec<String> {
    let mut root = TrieNode::new();

    for word in words {
        root.insert(word);
    }

    let rows = board.len();
    let cols = board[0].len();
    let mut path = HashSet::new();
    let mut res = HashSet::new();

    fn dfs(
        r: usize,
        c: usize,
        word: &mut String,
        node: &mut TrieNode,
        path: &mut HashSet<(usize, usize)>,
        res: &mut HashSet<String>,
        board: &[Vec<char>],
        rows: usize,
        cols: usize,
    ) {
        if r >= rows
            || c >= cols
            || !node.children.contains_key(&board[r][c])
            || path.contains(&(r, c))
        {
            return;
        }

        path.insert((r, c));
        word.push(board[r][c]);

        if node.is_word {
            res.insert(word.clone());
        }

        if let Some(child_node) = node.children.get_mut(&board[r][c]) {
            if child_node.is_word {
                res.insert(word.clone());
            }

            dfs(r + 1, c, word, child_node, path, res, board, rows, cols);
            dfs(
                r.saturating_sub(1),
                c,
                word,
                child_node,
                path,
                res,
                board,
                rows,
                cols,
            );
            dfs(r, c + 1, word, child_node, path, res, board, rows, cols);
            dfs(
                r,
                c.saturating_sub(1),
                word,
                child_node,
                path,
                res,
                board,
                rows,
                cols,
            );
        }

        path.remove(&(r, c));
        word.pop();
    }

    for r in 0..rows {
        for c in 0..cols {
            let mut word = String::new();
            dfs(
                r, c, &mut word, &mut root, &mut path, &mut res, &board, rows, cols,
            );
        }
    }

    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_two_vec_of_string(v1: Vec<String>, v2: Vec<&str>) {
        let mut v2 = v2.to_vec();
        v2.sort();
        let mut v1 = v1;
        v1.sort();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_find_words() {
        compare_two_vec_of_string(
            find_words(
                vec![
                    vec!['o', 'a', 'a', 'n'],
                    vec!['e', 't', 'a', 'e'],
                    vec!['i', 'h', 'k', 'r'],
                    vec!['i', 'f', 'l', 'v'],
                ],
                vec!["oath", "pea", "eat", "rain"],
            ),
            vec!["eat", "oath"],
        );
    }
}
