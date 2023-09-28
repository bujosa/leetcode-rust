#![allow(dead_code)]
pub fn find_min(nums: Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(find_min(nums), 1);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(find_min(nums), 0);
    }

    #[test]
    fn test_3() {
        let nums = vec![11, 13, 15, 17];
        assert_eq!(find_min(nums), 11);
    }

    #[test]
    fn test_4() {
        let nums = vec![2, 1];
        assert_eq!(find_min(nums), 1);
    }
}
