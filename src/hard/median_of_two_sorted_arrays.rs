#![allow(dead_code)]
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    0.000000
}

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
}
