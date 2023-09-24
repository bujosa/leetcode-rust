#![allow(dead_code)]

pub fn find_binary_gap(n: i32) -> i32 {
    let mut max_gap = 0;
    let mut current_gap = 0;
    let mut start_counting = false;
    let mut num = n;
    while num > 0 {
        if num & 1 == 1 {
            if !start_counting {
                start_counting = true;
            } else {
                max_gap = max_gap.max(current_gap);
            }
            current_gap = 0;
        } else if start_counting {
            current_gap += 1;
        }
        num >>= 1;
    }
    max_gap
}

/*
    Algorithm:
        - Bit Manipulation
    Time: O(logn)
    Space: O(1)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_binary_gap() {
        assert_eq!(find_binary_gap(9), 2);
        assert_eq!(find_binary_gap(529), 4);
        assert_eq!(find_binary_gap(20), 1);
        assert_eq!(find_binary_gap(15), 0);
        assert_eq!(find_binary_gap(32), 0);
        assert_eq!(find_binary_gap(1041), 5);
    }
}
