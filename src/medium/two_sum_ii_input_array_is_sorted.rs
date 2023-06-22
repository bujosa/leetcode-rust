#![allow(dead_code)]
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;
    while left < right {
        let sum = numbers[left] + numbers[right];
        if sum == target {
            return vec![left as i32 + 1, right as i32 + 1];
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    vec![]
}

/*
   Algorithm - Two Pointers
   - Initialize two pointers left and right pointing to the first and last element of the array respectively.
   - Loop until left < right
       - If the sum of the elements at left and right is equal to target, return the indices of the elements.
       - If the sum is less than target, increment left by 1.
       - If the sum is greater than target, decrement right by 1.
   - Return an empty vector if no such pair exists.

   Time O(N)
   Space O(1)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
