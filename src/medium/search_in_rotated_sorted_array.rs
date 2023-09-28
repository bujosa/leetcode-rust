#![allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(search(nums, target), 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(search(nums, target), -1);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        let target = 0;
        assert_eq!(search(nums, target), -1);
    }

    #[test]
    fn test_4() {
        let nums = vec![1, 3];
        let target = 3;
        assert_eq!(search(nums, target), 1);
    }

    #[test]
    fn test_5() {
        let nums = vec![3, 1];
        let target = 1;
        assert_eq!(search(nums, target), 1);
    }

    #[test]
    fn test_6() {
        let nums = vec![5, 1, 3];
        let target = 5;
        assert_eq!(search(nums, target), 0);
    }

    #[test]
    fn test_7() {
        let nums = vec![4, 5, 6, 7, 8, 1, 2, 3];
        let target = 8;
        assert_eq!(search(nums, target), 4);
    }

    #[test]
    fn test_8() {
        let nums = vec![4, 5, 6, 7, 8, 1, 2, 3];
        let target = 3;
        assert_eq!(search(nums, target), 7);
    }

    #[test]
    fn test_9() {
        let nums = vec![4, 5, 6, 7, 8, 1, 2, 3];
        let target = 4;
        assert_eq!(search(nums, target), 0);
    }
}
