#![allow(dead_code)]
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(
        nums: &Vec<i32>,
        start: usize,
        end: usize,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        result.push(path.clone());
        for i in start..end {
            path.push(nums[i]);
            backtrack(nums, i + 1, end, path, result);
            path.pop();
        }
    }

    let mut result = Vec::new();
    backtrack(&nums, 0, nums.len(), &mut Vec::new(), &mut result);
    result
}

/*
    Algorithm - Backtracking
    - Start with an empty set
    - Add each element to the set
    - Add the set to the result
    - Remove the element from the set
    - Repeat for each element

    Time Complexity: O(n * 2^n)
    Space Complexity: O(n * 2^n)

*/

#[cfg(test)]
mod tests {
    use super::*;

    fn validate_subsets(mut result: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) -> bool {
        result.sort();
        result == expected
    }

    #[test]
    fn test_subsets() {
        assert!(validate_subsets(
            subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3]
            ]
        ));
    }
}
