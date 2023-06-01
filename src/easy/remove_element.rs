#![allow(dead_code)]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut j = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[j] = nums[i];
            j += 1;
        }
    }
    nums.truncate(j);
    j as i32
}


#[test]
fn test_remove_element() {
    let mut nums = vec![3, 2, 2, 3];
    assert_eq!(remove_element(&mut nums, 3), 2);
    assert_eq!(nums, vec![2, 2]);

    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(remove_element(&mut nums, 2), 5);
    assert_eq!(nums, vec![0, 1, 3, 0, 4]);
}