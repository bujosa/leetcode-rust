#![allow(dead_code)]
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut output = vec![1; nums.len()];
    let mut left = 1;
    let mut right = 1;

    for i in 0..nums.len() {
        output[i] *= left;
        left *= nums[i];

        output[nums.len() - 1 - i] *= right;
        right *= nums[nums.len() - 1 - i];
    }

    output
}

/*
   Algorithm - Two Pass
   - Create an output array of the same size as the input array
   - Iterate through the input array from left to right
       - At each index, multiply the current element with the previous element
       - Store the result in the output array
   - Iterate through the input array from right to left
       - At each index, multiply the current element with the previous element
       - Store the result in the output array
   - Return the output array

   Time: O(n)
   Space: O(1), excluding the output array
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);

        assert_eq!(
            product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );

        assert_eq!(product_except_self(vec![1, 2]), vec![2, 1]);

        assert_eq!(product_except_self(vec![1]), vec![1]);
    }
}
