#![allow(dead_code)]
pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut max_count = 0;
    let mut start = 0;
    let mut max_length = 0;
    let mut map = [0; 26];

    for end in 0..s.len() {
        let c = s.chars().nth(end).unwrap();
        map[c as usize - 'A' as usize] += 1;
        max_count = max_count.max(map[c as usize - 'A' as usize]);

        while end - start + 1 - max_count > k as usize {
            let c = s.chars().nth(start).unwrap();
            map[c as usize - 'A' as usize] -= 1;
            start += 1;
        }

        max_length = max_length.max(end - start + 1);
    }

    max_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_replacement() {
        assert_eq!(character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(character_replacement("AABABBA".to_string(), 1), 4);
    }
}
