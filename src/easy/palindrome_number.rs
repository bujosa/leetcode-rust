pub fn palindrome_number(x: i32) -> bool {
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
    assert_eq!(palindrome_number(121), true);
    assert_eq!(palindrome_number(-121), false);
    assert_eq!(palindrome_number(10), false);
    assert_eq!(palindrome_number(-101), false);
}
