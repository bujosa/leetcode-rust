#![allow(dead_code)]
pub fn num_decodings(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let mut dp = vec![0; s.len() + 1];
    dp[s.len()] = 1;

    for i in (0..s.len()).rev() {
        if s[i] != '0' {
            dp[i] += dp[i + 1];
            if i + 1 < s.len() && (s[i] == '1' || (s[i] == '2' && s[i + 1] < '7')) {
                dp[i] += dp[i + 2];
            }
        }
    }

    dp[0]
}

#[test]
fn test_num_decodings() {
    assert_eq!(num_decodings("12".to_string()), 2);
    assert_eq!(num_decodings("226".to_string()), 3);
    assert_eq!(num_decodings("0".to_string()), 0);
    assert_eq!(num_decodings("06".to_string()), 0);
}
