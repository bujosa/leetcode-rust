#![allow(dead_code)]
pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] > nums[right] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    return nums[left];
}

/*
    Algorithm - Binary Search
     - Initialize left to 0 and right to the last index of the array
     - While left < right
      - Get the mid index
      - If the mid value is greater than the right value
       - Set left to mid + 1
      - Else
       - Set right to mid

     - Return the value at the left index
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(find_min(nums), 1);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(find_min(nums), 0);
    }

    #[test]
    fn test_3() {
        let nums = vec![11, 13, 15, 17];
        assert_eq!(find_min(nums), 11);
    }

    #[test]
    fn test_4() {
        let nums = vec![2, 1];
        assert_eq!(find_min(nums), 1);
    }
}
