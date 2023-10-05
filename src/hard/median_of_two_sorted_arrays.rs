#![allow(dead_code)]
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (nums1, nums2) = if nums1.len() > nums2.len() {
        (nums2, nums1)
    } else {
        (nums1, nums2)
    };

    let half = (nums1.len() + nums2.len() + 1) / 2;
    let (m, n) = (nums1.len(), nums2.len());
    let (mut left, mut right) = (0, m);

    while left <= right {
        let partition_x = (left + right) / 2;
        let partition_y = half - partition_x;

        // Partition_x is for m, Partition_y is for n
        let max_x = if partition_x == 0 {
            i32::MIN
        } else {
            nums1[partition_x - 1]
        };

        let min_x = if partition_x == m {
            i32::MAX
        } else {
            nums1[partition_x]
        };

        let max_y = if partition_y == 0 {
            i32::MIN
        } else {
            nums2[partition_y - 1]
        };

        let min_y = if partition_y == n {
            i32::MAX
        } else {
            nums2[partition_y]
        };

        // Three conditions
        if max_x <= min_y && max_y <= min_x {
            return calculate_median(max_x, max_y, min_x, min_y, m, n);
        } else if max_x > min_y {
            right = partition_x - 1;
        } else {
            left = partition_x + 1;
        }
    }
    0.0
}

fn calculate_median(max_x: i32, max_y: i32, min_x: i32, min_y: i32, m: usize, n: usize) -> f64 {
    if (m + n) % 2 == 0 {
        (max_x.max(max_y) + min_x.min(min_y)) as f64 / 2.0
    } else {
        max_x.max(max_y) as f64
    }
}

/*
    Algorithm - Binary Search + two pointers
        1. Find the middle element of the combined array
        2. If the combined array is even, find the average of the middle two elements
        3. If the combined array is odd, return the middle element

    Example:
    nums1 = [1, 3, 8, 9, 15]
    nums2 = [7, 11, 18, 19, 21, 25]

    median1 = 8

    Time Complexity - O(log(min(m, n)))
    Space Complexity - O(1)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.00000);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.50000);
    }

    #[test]
    fn test_find_median_sorted_arrays_2() {
        assert_eq!(find_median_sorted_arrays(vec![0, 0], vec![0, 0]), 0.00000);
        assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.00000);
    }

    #[test]
    fn test_find_median_sorted_arrays_3() {
        assert_eq!(
            find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![1, 2, 3, 4, 5]),
            4.00000
        );
    }
}
