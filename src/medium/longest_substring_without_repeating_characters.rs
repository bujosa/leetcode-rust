#![allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_length = 0;
    let mut start = 0;

    let mut map = std::collections::HashMap::new();

    for (end, c) in s.chars().enumerate() {
        if let Some(i) = map.get(&c) {
            start = std::cmp::max(start, *i + 1);
        }

        map.insert(c, end);

        max_length = std::cmp::max(max_length, end - start + 1);
    }

    max_length as i32
}

/*
    Algorithm - Sliding Window

    Time Complexity = O(n)
    Space Complexity = O(n)

    Explanation:
    1. Create a HashMap to store the characters and their indices
    2. Iterate through the string
    3. If the character is already in the HashMap, set the start to the maximum of the current start and the index of the character + 1
    4. Insert the character and its index into the HashMap
    Note: The HashMap stores the index of the character + 1 because if the character is already in the HashMap, the start should be set to the index of the character + 1
    5. Set the max_length to the maximum of the current max_length and the end - start + 1
    6. Return the max_length

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }
}
