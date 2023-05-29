use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if map.contains_key(&complement) {
            return vec![*map.get(&complement).unwrap() as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}