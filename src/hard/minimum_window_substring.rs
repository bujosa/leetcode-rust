#![allow(dead_code)]
pub fn min_window(s: String, t: String) -> String {
    if t.len() > s.len() {
        return "".to_string();
    }

    let need_map = t.chars().fold([0; 128], |mut acc, c| {
        acc[c as usize] += 1;
        acc
    });
    let mut window_map = [0; 128];
    let mut left = 0;
    let mut right = 0;
    let mut count = 0;
    let mut min_substring = "".to_string();

    while right < s.len() {
        let c = s.chars().nth(right).unwrap();
        window_map[c as usize] += 1;
        if window_map[c as usize] <= need_map[c as usize] {
            count += 1;
        }

        // If the window contains all the characters in t
        // Try to shrink the window
        while count == t.len() {
            let c = s.chars().nth(left).unwrap();
            if right - left + 1 < min_substring.len() || min_substring.is_empty() {
                min_substring = s[left..=right].to_string();
            }
            window_map[c as usize] -= 1;
            if window_map[c as usize] < need_map[c as usize] {
                count -= 1;
            }
            left += 1;
        }
        right += 1;
    }

    min_substring
}

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
