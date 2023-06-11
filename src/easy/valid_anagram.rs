#![allow(dead_code)]
pub fn is_anagram(s: String, t: String) -> bool {
    use std::collections::HashMap;

    let mut s_map: HashMap<char, i32> = HashMap::new();
    let mut t_map: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        let count = s_map.entry(c).or_insert(0);
        *count += 1;
    }

    for c in t.chars() {
        let count = t_map.entry(c).or_insert(0);
        *count += 1;
    }

    s_map == t_map
}

/*
  Algorithm - Hashmap
   - Use a hashmap to store the frequency of each character in the first string
   - Iterate through the second string and decrement the frequency of each character
   - If the frequency of a character is 0, remove it from the hashmap
   - If the hashmap is empty, then the strings are anagrams
   - This solution is O(n) time and O(n) space

  Time Complexity - O(n)
  Space Complexity - O(n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(
            is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);

        assert_eq!(is_anagram("aacc".to_string(), "ccac".to_string()), false);
    }
}
