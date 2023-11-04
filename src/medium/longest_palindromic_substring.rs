#![allow(dead_code)]
pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut res = String::new();

    for i in 0..s.len() {
        for &(l, r) in [(i, i), (i, i + 1)].iter() {
            let (mut l, mut r) = (l, r);
            while l <= r && r < s.len() && chars[l] == chars[r] {
                if r - l + 1 > res.len() {
                    res = s[l..=r].to_string();
                }
                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            }
        }
    }

    res
}

#[test]
fn test_longest_palindrome() {
    assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
    assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
    assert_eq!(longest_palindrome("a".to_string()), "a".to_string());
    assert_eq!(longest_palindrome("ac".to_string()), "a".to_string());
}