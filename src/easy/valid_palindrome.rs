#![allow(dead_code)]
pub fn is_palindrome(s: String) -> bool {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_not_palindrome() {
        assert_eq!(is_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(" ".to_string()), true);

        assert_eq!(is_palindrome("".to_string()), true);

        assert_eq!(
            is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }
}
