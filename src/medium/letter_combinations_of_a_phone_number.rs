#![allow(dead_code)]
pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let phone_map = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let mut output = Vec::new();

    fn backtrack(
        combination: String,
        next_digits: &str,
        phone_map: &Vec<&str>,
        output: &mut Vec<String>,
    ) {
        if next_digits.is_empty() {
            output.push(combination);
        } else {
            let letters = phone_map[next_digits.chars().nth(0).unwrap() as usize - '2' as usize];
            for letter in letters.chars() {
                let new_combination = combination.clone() + &letter.to_string();
                backtrack(new_combination, &next_digits[1..], phone_map, output);
            }
        }
    }

    backtrack(String::new(), &digits, &phone_map, &mut output);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        let result = letter_combinations("23".to_string());
        let expected = vec![
            "ad".to_string(),
            "ae".to_string(),
            "af".to_string(),
            "bd".to_string(),
            "be".to_string(),
            "bf".to_string(),
            "cd".to_string(),
            "ce".to_string(),
            "cf".to_string(),
        ];
        assert_eq!(result, expected);
    }
}
