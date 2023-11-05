#![allow(dead_code)]
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut dp = vec![1; nums.len()];
    for i in 0..nums.len() {
        for j in 0..i {
            if nums[i] > nums[j] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    *dp.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }
}
