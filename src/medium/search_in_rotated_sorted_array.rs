#![allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            return mid as i32;
        }

        if nums[mid] >= nums[left] {
            if target >= nums[left] && target < nums[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if target <= nums[right] && target > nums[mid] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    -1
}

/*
    Algorithm - Binary Search
     - Initialize left to 0 and right to the last index of the array
     - While left <= right
      - Get the mid index
      - If the mid value is equal to the target
       - Return the mid index
      - If the mid value is greater than or equal to the left value
       - If the target is greater than or equal to the left value and the target is less than the mid value
        - Set right to mid - 1
       - Else
        - Set left to mid + 1
      - Else
       - If the target is greater than or equal to the mid value and the target is less than or equal to the right value
        - Set left to mid + 1
       - Else
        - Set right to mid - 1

     - Return -1
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(search(nums, target), 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(search(nums, target), -1);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        let target = 0;
        assert_eq!(search(nums, target), -1);
    }

    #[test]
    fn test_4() {
        let nums = vec![1, 3];
        let target = 3;
        assert_eq!(search(nums, target), 1);
    }

    #[test]
    fn test_5() {
        let nums = vec![3, 1];
        let target = 1;
        assert_eq!(search(nums, target), 1);
    }

    #[test]
    fn test_6() {
        let nums = vec![5, 1, 3];
        let target = 5;
        assert_eq!(search(nums, target), 0);
    }

    #[test]
    fn test_7() {
        let nums = vec![4, 5, 6, 7, 8, 1, 2, 3];
        let target = 8;
        assert_eq!(search(nums, target), 4);
    }

    #[test]
    fn test_8() {
        let nums = vec![4, 5, 6, 7, 8, 1, 2, 3];
        let target = 3;
        assert_eq!(search(nums, target), 7);
    }

    #[test]
    fn test_9() {
        let nums = vec![4, 5, 6, 7, 8, 1, 2, 3];
        let target = 4;
        assert_eq!(search(nums, target), 0);
    }
}
