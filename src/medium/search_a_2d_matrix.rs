#![allow(dead_code)]
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let (first_value, last_value) = (matrix[0][0], matrix[matrix.len() - 1][matrix[0].len() - 1]);

    if target < first_value || target > last_value {
        return false;
    }

    let (m, n) = (matrix.len(), matrix[0].len());
    let (mut left, mut right) = (0, m * n - 1);

    while left <= right {
        let mid = left + (right - left) / 2;
        let (i, j) = (mid / n, mid % n);
        if matrix[i][j] == target {
            return true;
        } else if matrix[i][j] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    false
}

/*
     Algorithm - Binary Search
        - Time Complexity: O(log(mn))
        - Space Complexity: O(1)]

    Steps:
        1. Check if the matrix is empty or the first value is greater than the target or the last value is less than the target
        2. If not, then perform binary search on the matrix
        3. If the value is found, return true
        4. If not, return false
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

    #[test]
    fn test_search_matrix_2() {
        assert_eq!(search_matrix(vec![vec![1]], 0), false);
    }

    #[test]
    fn test_search_matrix_3() {
        assert_eq!(
            search_matrix(vec![vec![-9, -9, -7], vec![-4, -2, 0], vec![3, 3, 3]], -14),
            false
        );
    }
}
