#![allow(dead_code)]
pub fn my_sqrt(x: i32) -> i32 {
    let mut left = 0;
    let mut right = x;
    let mut ans = -1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if mid > 0 && mid > x / mid {
            right = mid - 1;
        } else {
            ans = mid;
            left = mid + 1;
        }
    }

    ans
}

/*
    Algorithm:
        - Binary Search
        - Time Complexity: O(log n)
        - Space Complexity: O(1)
 */

#[test]
fn test_my_sqrt() {
    assert_eq!(my_sqrt(0), 0);
    assert_eq!(my_sqrt(1), 1);
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(8), 2);
    assert_eq!(my_sqrt(9), 3);
    assert_eq!(my_sqrt(10), 3);
    assert_eq!(my_sqrt(30), 5);
    assert_eq!(my_sqrt(2147395599), 46339);
}