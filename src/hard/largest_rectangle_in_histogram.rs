#![allow(dead_code)]
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_area() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(largest_rectangle_area(heights), 10);
    }
}
