#![allow(dead_code)]
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m;
    let mut n = n;
    while m + n > 0 {
        if m == 0 {
            nums1[(m + n - 1) as usize] = nums2[(n - 1) as usize];
            n -= 1;
        } else if n == 0 {
            nums1[(m + n - 1) as usize] = nums1[(m - 1) as usize];
            m -= 1;
        } else if nums1[(m - 1) as usize] > nums2[(n - 1) as usize] {
            nums1[(m + n - 1) as usize] = nums1[(m - 1) as usize];
            m -= 1;
        } else {
            nums1[(m + n - 1) as usize] = nums2[(n - 1) as usize];
            n -= 1;
        }
    }
}

/*
    Algorithm:
        - Use two pointers, one for each array
        - Compare the values at the pointers
        - If the value at the first pointer is greater than the value at the second pointer, swap the values
        - Increment the pointer for the second array
        - Repeat until the second pointer is at the end of the second array
        - Return the first array

    Time complexity:
        - O(n + m) where n is the length of the first array and m is the length of the second array
    
    Space complexity:
        - O(1)
    
 */

#[test]
fn test_merge() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}