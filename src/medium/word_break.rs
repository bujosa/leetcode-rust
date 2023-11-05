#![allow(dead_code)]
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let word_set: std::collections::HashSet<_> = word_dict.into_iter().collect();
    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;

    for i in 1..=s.len() {
        for j in 0..i {
            if dp[j] && word_set.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }

    dp[s.len()]
}

#[test]
fn test_word_break() {
    assert_eq!(
        word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ),
        true
    );
    assert_eq!(
        word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()]
        ),
        true
    );
    assert_eq!(
        word_break(
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string()
            ]
        ),
        false
    );
}
