#![allow(dead_code)]
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut left = 0;
    let mut right = m * n - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        let mid_val = matrix[mid / n][mid % n];
        if mid_val == target {
            return true;
        } else if mid_val < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    false
}

/*
    Approach: Binary Search
    - Treat the matrix as a sorted array
    - Use binary search to find the target
    - Time Complexity: O(log(mn))
    - Space Complexity: O(1)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix() {
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            ),
            true
        );
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            ),
            false
        );
    }
}
