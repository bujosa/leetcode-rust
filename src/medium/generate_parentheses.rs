#![allow(dead_code)]
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut results = Vec::new();

    fn helper(results: &mut Vec<String>, s: String, open: i32, close: i32, n: i32) {
        if open == n && close == n {
            results.push(s);
        } else {
            if open < n {
                helper(results, s.clone() + "(", open + 1, close, n);
            }
            if close < open {
                helper(results, s.clone() + ")", open, close + 1, n);
            }
        }
    }

    helper(&mut results, "".to_string(), 0, 0, n);

    results
}

/*
    Algorithm - Backtracking
    - Create a vector to store the results
    - Create a helper function to generate the parenthesis
        - If the number of open parenthesis is equal to the number of close parenthesis and the number of open parenthesis is equal to n
            - Push the current string onto the results vector
        - If the number of open parenthesis is less than n
            - Call the helper function with the current string + "(" and the number of open parenthesis + 1
        - If the number of close parenthesis is less than the number of open parenthesis
            - Call the helper function with the current string + ")" and the number of close parenthesis + 1
    - Call the helper function with the current string and the number of open and close parenthesis set to 0
    - Return the results vector

    Time: O(2^n)
    Space: O(2^n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(
            generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }
}
