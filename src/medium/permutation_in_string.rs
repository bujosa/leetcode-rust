#![allow(dead_code)]
pub fn check_inclusion(s1: String, s2: String) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_inclusion() {
        assert_eq!(
            check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );

        assert_eq!(
            check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );

        assert_eq!(check_inclusion("adc".to_string(), "dcda".to_string()), true);
    }
}
