#![allow(dead_code)]
pub fn check_inclusion(s1: String, s2: String) -> bool {
    let mut s1_map = [0; 26];
    let mut s2_map = [0; 26];
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let s1_len = s1.len();
    let s2_len = s2.len();

    if s1_len > s2_len {
        return false;
    }

    for i in 0..s1_len {
        s1_map[(s1_bytes[i] - b'a') as usize] += 1;
        s2_map[(s2_bytes[i] - b'a') as usize] += 1;
    }

    for i in 0..(s2_len - s1_len) {
        if s1_map == s2_map {
            return true;
        }
        s2_map[(s2_bytes[i] - b'a') as usize] -= 1;
        s2_map[(s2_bytes[i + s1_len] - b'a') as usize] += 1;
    }

    s1_map == s2_map
}

/* Algorithm - Sliding Window

   Time Complexity - O(n)
    Space Complexity - O(1)

 * 1. Create two arrays of size 26 to store the count of each character in s1 and s2.
 * 2. If the length of s1 is greater than s2, return false.
 * 3. Iterate through s1 and s2 and store the count of each character in the respective arrays.
 * 4. Iterate through s2 and check if the count of each character in s1 and s2 are equal.
 * 5. If the count of each character in s1 and s2 are equal, return true.
 * 6. If the count of each character in s1 and s2 are not equal, return false.
 *
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_inclusion() {
        assert_eq!(
            check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );

        assert_eq!(
            check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );

        assert_eq!(check_inclusion("adc".to_string(), "dcda".to_string()), true);
    }
}
