#![allow(dead_code)]
pub fn climb_stairs(n: i32) -> i32 {
    let mut dp = vec![0; n as usize + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n as usize {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n as usize]
}

/*
    Algorithm:
        - Dynamic Programming
    Time: O(n)
    Space: O(n)
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
        assert_eq!(climb_stairs(5), 8);
        assert_eq!(climb_stairs(6), 13);
        assert_eq!(climb_stairs(7), 21);
        assert_eq!(climb_stairs(8), 34);
        assert_eq!(climb_stairs(9), 55);
    }
}