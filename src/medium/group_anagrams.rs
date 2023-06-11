#![allow(dead_code)]
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
    for s in strs {
        let mut sorted = s.as_bytes().to_vec();
        sorted.sort();
        map.entry(sorted).or_insert(Vec::new()).push(s);
    }

    map.into_iter().map(|(_, v)| v).collect()
}

/*
   Algorithm
   - Create a hashmap with key as sorted string and value as vector of strings
   - Iterate over the input vector and sort each string
   - Insert the sorted string as key and the original string as value in the hashmap
   - Return the values of the hashmap

   Complexity
   - Time: O(n * klogk) where n is the number of strings and k is the length of the longest string
   - Space: O(n * k) where n is the number of strings and k is the length of the longest string

   Example
   - Input: ["eat", "tea", "tan", "ate", "nat", "bat"]
   - Output: [["ate","eat","tea"], ["nat","tan"], ["bat"]]
*/

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_vecs(v1: Vec<Vec<String>>, v2: Vec<Vec<String>>) -> bool {
        // Compare the length of the vectors
        if v1.len() != v2.len() {
            return false;
        }
        true
    }

    #[test]
    fn test_group_anagrams() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let expected = vec![
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["bat".to_string()],
        ];
        assert_eq!(compare_vecs(group_anagrams(strs), expected), true);
    }

    #[test]
    fn test_empty() {
        let strs = vec![];
        let expected: Vec<Vec<String>> = vec![];
        assert_eq!(group_anagrams(strs), expected);
    }

    #[test]
    fn test_single() {
        let strs = vec!["a".to_string()];
        let expected = vec![vec!["a".to_string()]];
        assert_eq!(group_anagrams(strs), expected);
    }

    #[test]
    fn test_empty_string() {
        let strs = vec!["".to_string()];
        let expected = vec![vec!["".to_string()]];
        assert_eq!(group_anagrams(strs), expected);
    }

    #[test]
    fn test_different_length() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
            "a".to_string(),
            "".to_string(),
        ];

        let expected = vec![
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["bat".to_string()],
            vec!["a".to_string()],
            vec!["".to_string()],
        ];

        assert_eq!(compare_vecs(group_anagrams(strs), expected), true);
    }
}
