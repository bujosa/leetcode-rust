#![allow(dead_code)]
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }
}
