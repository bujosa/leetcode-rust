#![allow(dead_code)]
pub fn count_substrings(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut count = 0;

    for i in 0..s.len() {
        for &(l, r) in [(i, i), (i, i + 1)].iter() {
            let (mut l, mut r) = (l, r);
            while l <= r && r < s.len() && chars[l] == chars[r] {
                count += 1;
                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            }
        }
    }

    count
}


#[test]
fn test_count_substrings() {
    assert_eq!(count_substrings("abc".to_string()), 3);
    assert_eq!(count_substrings("aaa".to_string()), 6);
}