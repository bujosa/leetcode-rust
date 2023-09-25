#![allow(dead_code)]
pub fn add_binary(a: String, b: String) -> String {
    let mut a = a.chars().rev();
    let mut b = b.chars().rev();

    let mut carry = 0;

    let mut result = String::new();

    loop {
        match (a.next(), b.next()) {
            (None, None) => {
                if carry == 1 {
                    result.push('1');
                }
                break;
            }
            (Some(a), None) => {
                let sum = a.to_digit(10).unwrap() + carry;
                if sum == 2 {
                    result.push('0');
                    carry = 1;
                } else {
                    result.push_str(&sum.to_string());
                    carry = 0;
                }
            }
            (None, Some(b)) => {
                let sum = b.to_digit(10).unwrap() + carry;
                if sum == 2 {
                    result.push('0');
                    carry = 1;
                } else {
                    result.push_str(&sum.to_string());
                    carry = 0;
                }
            }
            (Some(a), Some(b)) => {
                let sum = a.to_digit(10).unwrap() + b.to_digit(10).unwrap() + carry;
                if sum == 2 {
                    result.push('0');
                    carry = 1;
                } else if sum == 3 {
                    result.push('1');
                    carry = 1;
                } else {
                    result.push_str(&sum.to_string());
                    carry = 0;
                }
            }
        }
    }

    result.chars().rev().collect()
}

/*
   Algorithm:
       - Convert the strings to chars and reverse them
       - Loop through the chars and add them together
       - If the sum is 2, add 0 to the result and carry 1
       - If the sum is 3, add 1 to the result and carry 1
       - If the sum is 1, add 1 to the result and carry 0
       - If the sum is 0, add 0 to the result and carry 0
       - If there is a carry at the end, add 1 to the result
       - Reverse the result and return it

   Time complexity:
       - O(n) where n is the length of the longest string

   Space complexity:
       - O(n) where n is the length of the longest string



*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }
}
