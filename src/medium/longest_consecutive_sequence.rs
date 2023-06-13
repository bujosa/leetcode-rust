#![allow(dead_code)]
use std::{collections::HashSet, iter::FromIterator};

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = HashSet::from_iter(nums.into_iter());

    let mut max = 0;

    for n in &set {
        if !set.contains(&(n - 1)) {
            let mut next = n + 1;
            let mut count = 1;
            while set.contains(&next) {
                count += 1;
                next += 1;
            }

            max = max.max(count);
        }
    }
    max
}

/*
   Algorithm - HashSet
   - Create a hashset from the given vector
   - Iterate through the hashset
       - If the current element - 1 is not in the hashset
           - Create a counter and set it to 1
           - Create a next variable and set it to current element + 1
           - While the next variable is in the hashset
               - Increment the counter
               - Increment the next variable
           - Set the max counter to the max of the current max counter and the counter
   - Return the max counter

   Time: O(n)
   Space: O(n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_128() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);

        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}
