#![allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {
    let mut reversed = 0;
    let mut a = x;
    while a > 0 {
        reversed = reversed * 10 + a % 10;
        a = a / 10;
    }
    return x == reversed;
}

#[test]
fn test_palindrome_number() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(10), false);
    assert_eq!(is_palindrome(-101), false);
}
