#![allow(dead_code)]
use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for num in nums {
        if set.contains(&num) {
            return true;
        }
        set.insert(num);
    }
    false
}

/*
   Algorithm - Hash Set
   - Create a hash set
   - Iterate through the vector
       - If the hash set contains the current number
           - Return true
       - Else
           - Insert the current number into the hash set
   - Return false

   Time Complexity = O(n)
   Space Complexity = O(n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}
