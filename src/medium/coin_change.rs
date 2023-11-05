#![allow(dead_code)]
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![amount + 1; (amount + 1) as usize];
    dp[0] = 0;

    for a in 1..=amount {
        for &c in &coins {
            if c <= a {
                dp[a as usize] = dp[a as usize].min(dp[(a - c) as usize] + 1);
            }
        }
    }

    if dp[amount as usize] > amount {
        -1
    } else {
        dp[amount as usize]
    }
}

#[test]
fn test_coin_change() {
    assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(coin_change(vec![2], 3), -1);
    assert_eq!(coin_change(vec![1], 0), 0);
    assert_eq!(coin_change(vec![1], 1), 1);
    assert_eq!(coin_change(vec![1], 2), 2);
}
