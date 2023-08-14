#![allow(dead_code)]
pub fn min_window(s: String, t: String) -> String {
    "".to_string()
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
}
