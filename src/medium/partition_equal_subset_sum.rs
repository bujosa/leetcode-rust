#![allow(dead_code)]
pub fn can_partition(nums: Vec<i32>) -> bool {
    let total: i32 = nums.iter().sum();
    if total % 2 != 0 {
        return false;
    }
    let target = total / 2;
    let mut dp = vec![false; (target + 1) as usize];
    dp[0] = true;
    for &num in &nums {
        for i in (num as usize..=target as usize).rev() {
            dp[i] = dp[i] || dp[i - num as usize];
        }
    }
    dp[target as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_partition() {
        assert_eq!(can_partition(vec![1, 5, 11, 5]), true);
        assert_eq!(can_partition(vec![1, 2, 3, 5]), false);
        assert_eq!(can_partition(vec![1, 2]), false);
        assert_eq!(can_partition(vec![1, 2, 3, 4, 5, 6, 7]), true);
        assert_eq!(can_partition(vec![1, 2, 3, 4, 5, 6, 7, 8]), true);
    }
}
