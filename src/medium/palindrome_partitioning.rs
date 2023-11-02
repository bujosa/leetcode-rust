#![allow(dead_code)]
pub fn partition(s: String) -> Vec<Vec<String>> {
    let mut res = Vec::new();
    let s: Vec<char> = s.chars().collect();

    fn is_palindrome(s: &[char]) -> bool {
        let rev: Vec<char> = s.iter().rev().cloned().collect();
        s == &rev[..]
    }

    fn backtracking(start: usize, path: &mut Vec<String>, s: &[char], res: &mut Vec<Vec<String>>) {
        if start == s.len() {
            res.push(path.clone());
            return;
        }
        for end in start + 1..=s.len() {
            if is_palindrome(&s[start..end]) {
                path.push(s[start..end].iter().collect());
                backtracking(end, path, s, res);
                path.pop();
            }
        }
    }

    backtracking(0, &mut Vec::new(), &s, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        assert_eq!(
            partition("aab".to_string()),
            vec![
                vec!["a".to_string(), "a".to_string(), "b".to_string()],
                vec!["aa".to_string(), "b".to_string()],
            ]
        );
    }
}
