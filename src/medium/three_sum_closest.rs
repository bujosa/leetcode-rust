#![allow(dead_code)]
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut closest = nums[0] + nums[1] + nums[2];

    for i in 0..nums.len() - 2 {
        let mut j = i + 1;
        let mut k = nums.len() - 1;

        while j < k {
            let sum = nums[i] + nums[j] + nums[k];
            if (sum - target).abs() < (closest - target).abs() {
                closest = sum;
            }
            if sum > target {
                k -= 1;
            } else {
                j += 1;
            }
        }
    }
    closest
}

/*
    Algorithm - Two Pointers
        - Time Complexity: O(n^2)
        - Space Complexity: O(1)
*/

#[test]
fn test_three_sum_closest() {
    assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
}

// Reference: https://leetcode.com/problems/3sum-closest
