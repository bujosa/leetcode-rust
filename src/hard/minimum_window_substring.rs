#![allow(dead_code)]

use std::collections::HashMap;
pub fn min_window(s: String, t: String) -> String {
    let s: Vec<char> = s.chars().collect();

    if t.is_empty() || s.len() < t.len() {
        return String::new();
    }

    let mut l = 0;
    let mut res = (-1, -1);
    let mut res_len = usize::MAX;
    let mut count_t: HashMap<char, usize> = HashMap::new();
    let mut window: HashMap<char, _> = HashMap::new();

    for c in t.chars() {
        *count_t.entry(c).or_default() += 1;
    }

    let need = count_t.len();
    let mut have = 0;

    for r in 0..s.len() {
        let c = s[r];

        *window.entry(c).or_default() += 1;
        have += (window.get(&c) == count_t.get(&c)) as usize;

        while have == need {
            if (r - l + 1) < res_len {
                res = (l as i32, r as i32);
                res_len = r - l + 1;
            }
            *window.get_mut(&s[l]).unwrap() -= 1;

            if window.get(&s[l]) < count_t.get(&s[l]) {
                have -= 1;
            }

            l += 1;
        }
    }

    if res.0 > -1 && res.1 > -1 {
        return s[res.0 as usize..=res.1 as usize].into_iter().collect();
    }

    String::new()
}

/*
    Algorithm - Sliding Window
    Time complexity: O(n)
    Space complexity: O(n)

    - Create a hashmap of the characters in t and their count
    - Create a hashmap of the characters in the window and their count
    - Create a variable to keep track of the number of unique characters in t
    - Create a variable to keep track of the number of unique characters in the window
    - Create a variable to keep track of the left index of the window
    - Create a variable to keep track of the right index of the window
    - Create a variable to keep track of the minimum window length
    - Create a variable to keep track of the minimum window indices
    - Iterate through the string
        - Add the current character to the window hashmap
        - If the count of the current character in the window hashmap is equal to the count of the current character in the t hashmap, increment the number of unique characters in the window
        - While the number of unique characters in the window is equal to the number of unique characters in t
            - If the current window length is less than the minimum window length, update the minimum window length and indices
            - Remove the leftmost character from the window hashmap
            - If the count of the leftmost character in the window hashmap is less than the count of the leftmost character in the t hashmap, decrement the number of unique characters in the window
            - Increment the left index of the window
    - If the minimum window indices are valid, return the substring of the minimum window indices
    - Otherwise, return an empty string

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        assert_eq!(min_window(s, t), "BANC".to_string());
    }

    #[test]
    fn test_min_window_1() {
        let s = "a".to_string();
        let t = "aa".to_string();
        assert_eq!(min_window(s, t), "".to_string());
    }

    #[test]
    fn test_min_window_2() {
        let s = "a".to_string();
        let t = "a".to_string();
        assert_eq!(min_window(s, t), "a".to_string());
    }
}
