#![allow(dead_code)]
pub fn jump(nums: Vec<i32>) -> i32 {
    let mut min_jump = 0;
    let mut l = 0;
    let mut r = 0;

    while r < nums.len() - 1 {
        let mut farthest = 0;

        for i in l..=r {
            farthest = farthest.max(i + nums[i] as usize);
        }

        l = r + 1;
        r = farthest;
        min_jump += 1;
    }

    min_jump
}

#[test]
fn test_jump() {
    assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
}
