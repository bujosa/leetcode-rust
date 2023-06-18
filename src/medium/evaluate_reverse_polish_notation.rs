#![allow(dead_code)]
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();

    for token in tokens {
        match token.as_str() {
            "+" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            "-" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
            }
            "*" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            }
            "/" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b / a);
            }
            _ => {
                stack.push(token.parse::<i32>().unwrap());
            }
        }
    }

    stack.pop().unwrap()
}

/*
    Algorithm - Stack
    - Create a stack
    - Iterate through the tokens
        - If the current token is an operator
            - Pop the top two elements from the stack
            - Perform the operation on the two elements
            - Push the result back onto the stack
        - Else
            - Push the current token onto the stack
    - Return the top element of the stack

    Time: O(n)
    Space: O(n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_150() {
        assert_eq!(
            eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        );

        assert_eq!(
            eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ]),
            6
        );

        assert_eq!(
            eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string()
            ]),
            22
        );
    }
}
