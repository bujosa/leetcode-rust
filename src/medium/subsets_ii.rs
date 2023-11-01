#![allow(dead_code)]

pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    fn backtrack(
        nums: &Vec<i32>,
        start: usize,
        end: usize,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        result.push(path.clone());
        for i in start..end {
            // Skip duplicates
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            path.push(nums[i]);
            backtrack(nums, i + 1, end, path, result);
            path.pop();
        }
    }

    let mut result = Vec::new();
    backtrack(&nums, 0, nums.len(), &mut Vec::new(), &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets_with_dup() {
        let nums = vec![1, 2, 2];
        let expected = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2],
        ];
        assert_eq!(subsets_with_dup(nums), expected);
    }

    #[test]
    fn test_subsets_with_dup_2() {
        let nums = vec![1, 2, 2, 3];
        let expected = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![1, 2, 2, 3],
            vec![1, 2, 3],
            vec![1, 3],
            vec![2],
            vec![2, 2],
            vec![2, 2, 3],
            vec![2, 3],
            vec![3],
        ];
        assert_eq!(subsets_with_dup(nums), expected);
    }
}
