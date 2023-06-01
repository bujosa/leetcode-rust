#[allow(dead_code)]
pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();

    if needle.len() == 0 {
        return 0;
    }

    let mut i = 0;
    let mut j = 0;

    while i < haystack.len() {
        if haystack[i] == needle[j] {
            i += 1;
            j += 1;
            if j == needle.len() {
                return (i - j) as i32;
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
    -1
}

/*
 Algorithm: KMP (Knuth-Morris-Pratt) Algorithm

    1. Build a next array for needle string
    2. Compare haystack and needle string

    Time: O(m + n)

    Space: O(n)

    m: haystack length
    n: needle length

    Reference: https://www.youtube.com/watch?v=GTJr8OvyEVQ
 */

#[test]
fn test_str_str() {
    assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
    assert_eq!(str_str("aaaaa".to_string(), "bba".to_string()), -1);
    assert_eq!(str_str("".to_string(), "".to_string()), 0);
    assert_eq!(str_str("a".to_string(), "".to_string()), 0);
    assert_eq!(str_str("".to_string(), "a".to_string()), -1);
}