#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    stack.is_empty()
}

/*
    Algorithm: Using a stack
    - Create a stack
    - Iterate through the string
    - If the character is an opening bracket, push it onto the stack
    - If the character is a closing bracket, pop the stack and compare the popped character with the current character
    - If the popped character is not the corresponding opening bracket, return false
    - If the stack is empty, return true
    - Else, return false

    Time: O(n)
    Space: O(n)

 */

#[test]
fn test_is_valid() {
    assert_eq!(is_valid("()".to_string()), true);
    assert_eq!(is_valid("()[]{}".to_string()), true);
    assert_eq!(is_valid("(]".to_string()), false);
    assert_eq!(is_valid("([)]".to_string()), false);
    assert_eq!(is_valid("{[]}".to_string()), true);
}