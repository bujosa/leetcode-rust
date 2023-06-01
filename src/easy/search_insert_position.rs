#[allow(dead_code)]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {

    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut mid = 0;

    if target < nums[0] {
        return 0;
    } else if target > nums[nums.len() - 1] {
        return nums.len() as i32;
    }

    while left <= right {
        mid = (left + right) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] > target && mid > 0 {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    if nums[mid] > target {
        mid as i32
    } else {
        (mid + 1) as i32
    }
        
}

/*
    Algorithm:
        - Binary Search
    Time: O(log(n))
    Space: O(1)

    ref: https://leetcode.com/problems/search-insert-position
 */

#[test]
fn test_search_insert() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
    assert_eq!(search_insert(vec![1, 3], 0), 0);
    assert_eq!(search_insert(vec![1], 0), 0);
    assert_eq!(search_insert(vec![1], 2), 1);
}