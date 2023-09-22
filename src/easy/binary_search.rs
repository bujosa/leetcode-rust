#![allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {}

/*
    Algorithm:
        - Binary Search
    Time: O(logn)
    Space: O(1)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
