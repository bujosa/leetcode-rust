#![allow(dead_code)]
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut results = Vec::new();
    let mut stack = Vec::new();

    for i in 0..temperatures.len() {
        while !stack.is_empty() && temperatures[i] > temperatures[*stack.last().unwrap()] {
            let index = stack.pop().unwrap();
            results[index] = (i - index) as i32;
        }
        stack.push(i);
        results.push(0);
    }

    results
}

/*
    Algorithm - Stack
    - Create a vector to store the results
    - Create a stack to store the indices of the temperatures
    - Iterate through the temperatures
        - While the stack is not empty and the current temperature is greater than the temperature at the top of the stack
            - Pop the top of the stack and set the result at the index to the current index minus the index at the top of the stack
        - Push the current index onto the stack
    - Return the results vector

    Time: O(n)
    Space: O(n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_739() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}
