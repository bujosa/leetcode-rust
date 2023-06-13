#![allow(dead_code)]
pub fn encode(strs: Vec<String>) -> String {
    let mut result = String::new();
    for s in strs {
        result.push_str(&format!("{}{}", s.len(), s));
    }
    result
}

pub fn decode(s: String) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < s.len() {
        let mut j = i;
        while j < s.len() && s.chars().nth(j).unwrap().is_digit(10) {
            j += 1;
        }
        let len = s[i..j].parse::<usize>().unwrap();
        result.push(s[j..j + len].to_string());
        i = j + len;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let strs = vec![
            "Hello".to_string(),
            "World".to_string(),
            "How".to_string(),
            "Are".to_string(),
            "You".to_string(),
        ];
        assert_eq!(encode(strs), "5Hello5World3How3Are3You".to_string());
    }

    #[test]
    fn test_decode() {
        let s = "5Hello5World3How3Are3You".to_string();
        assert_eq!(
            decode(s),
            vec![
                "Hello".to_string(),
                "World".to_string(),
                "How".to_string(),
                "Are".to_string(),
                "You".to_string(),
            ]
        );
    }
}
