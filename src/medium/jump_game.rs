#![allow(dead_code)]
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut goal = nums.len() - 1;

    for i in (0..=goal).rev() {
        if i + nums[i] as usize >= goal {
            goal = i;
        }
    }

    goal == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_55() {
        assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
