#![allow(dead_code)]
pub fn is_anagram(s: String, t: String) -> bool {
    use std::collections::HashMap;

    let mut map: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    for c in t.chars() {
        let count = map.entry(c).or_insert(0);
        *count -= 1;
    }

    map.values().all(|&x| x == 0)
}

/*
  Algorithm - Hashmap
   - Create a hashmap
    - Iterate through the first string
         - Insert each character into the hashmap with a value of 1
    - Iterate through the second string
           - Insert each character into the hashmap with a value of -1
    - Iterate through the hashmap values
        - If any value is not 0
            - Return false
    - Return true

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
