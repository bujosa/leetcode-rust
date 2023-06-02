#[allow(dead_code)]
pub fn length_of_last_word(s: String) -> i32 {
    let mut count = 0;
    let mut last_word = false;
    for c in s.chars().rev() {
        if c == ' ' {
            if last_word {
                break;
            }
        } else {
            last_word = true;
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(length_of_last_word(" ".to_string()), 0);
        assert_eq!(length_of_last_word("a ".to_string()), 1);
        assert_eq!(length_of_last_word("a".to_string()), 1);
        assert_eq!(length_of_last_word("".to_string()), 0);
        assert_eq!(length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
    }
}