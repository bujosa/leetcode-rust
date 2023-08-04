#![allow(dead_code)]
pub fn character_replacement(s: String, k: i32) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_replacement() {
        assert_eq!(character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(character_replacement("AABABBA".to_string(), 1), 4);
    }
}
