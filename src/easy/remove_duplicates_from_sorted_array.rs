#[allow(dead_code)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut i = 0;
    for j in 1..nums.len() {
        if nums[i] != nums[j] {
            i += 1;
            nums[i] = nums[j];
        }
    }
    nums.truncate(i + 1);
    (i + 1) as i32
}

/*
    Algorithm:
        - If the length of the array is 0, return 0
        - Initialize a variable i to 0
        - Iterate through the array from 1 to the length of the array
            - If the value at index i is not equal to the value at index j
                - Increment i by 1
                - Set the value at index i to the value at index j
        - Truncate the array to i + 1
        - Return i + 1 as an i32
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(remove_duplicates(&mut vec![1,1,2]), 2);
        assert_eq!(remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]), 5);
    }

    #[test]
    fn test_remove_duplicates_length() {
        let mut nums = vec![1,1,2];
        remove_duplicates(&mut nums);
        assert_eq!(nums.len(), 2);
    }
}