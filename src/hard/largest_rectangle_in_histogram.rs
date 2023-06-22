#![allow(dead_code)]
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut stack = vec![];
    let mut max_area = 0;
    let mut i = 0;

    while i < heights.len() {
        if stack.is_empty() || heights[*stack.last().unwrap()] <= heights[i] {
            stack.push(i);
            i += 1;
        } else {
            let top = stack.pop().unwrap();
            let area = heights[top]
                * if stack.is_empty() {
                    i
                } else {
                    i - stack.last().unwrap() - 1
                } as i32;
            max_area = max_area.max(area);
        }
    }

    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        let area = heights[top]
            * if stack.is_empty() {
                i
            } else {
                i - stack.last().unwrap() - 1
            } as i32;
        max_area = max_area.max(area);
    }

    max_area
}

/*
    Algorithm - Stack
     - Push index to stack if stack is empty or current height is greater than or equal to the height at index at top of stack
        - If current height is less than the height at index at top of stack, pop the index from stack and calculate area
        - Area = height[top] * (stack.is_empty() ? i : i - stack.top() - 1)
        - Update max_area
    - Pop all the indexes from stack and calculate area
        - Area = height[top] * (stack.is_empty() ? i : i - stack.top() - 1)
        - Update max_area
    - Return max_area
    
    Time Complexity - O(n)
    Space Complexity - O(n)
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_area() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(largest_rectangle_area(heights), 10);
    }
}
