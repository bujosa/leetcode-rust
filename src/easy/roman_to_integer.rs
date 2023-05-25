// Roman to Interger
pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    let mut prev = 0;
    for c in s.chars() {
        let curr = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        result += if curr > prev { curr - 2 * prev } else { curr };
        prev = curr;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_13() {
        assert_eq!(roman_to_int(String::from("III")), 3);
        assert_eq!(roman_to_int(String::from("IV")), 4);
        assert_eq!(roman_to_int(String::from("IX")), 9);
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }
}

// Reference: https://leetcode.com/problems/roman-to-integer/
