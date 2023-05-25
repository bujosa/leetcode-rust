pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return "".to_string();
    }
    let mut prefix = strs[0].clone();
    for i in 1..strs.len() {
        let mut j = 0;
        while j < prefix.len() && j < strs[i].len() {
            if prefix.chars().nth(j).unwrap() != strs[i].chars().nth(j).unwrap() {
                break;
            }
            j += 1;
        }
        prefix = prefix[..j].to_string();
    }
    prefix
}

// Reference: https://leetcode.com/problems/longest-common-prefix/
