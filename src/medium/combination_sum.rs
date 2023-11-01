#![allow(dead_code)]

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn backtracking(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target < 0 {
            return;
        } else if target == 0 {
            result.push(path.clone());
        } else {
            for i in start..candidates.len() {
                path.push(candidates[i]);
                backtracking(candidates, target - candidates[i], i, path, result);
                path.pop();
            }
        }
    }

    let mut result = Vec::new();
    backtracking(&candidates, target, 0, &mut Vec::new(), &mut result);
    return result;
}

/*
    Backtracking Algorithm
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

    #[test]
    fn test_combination_sum_39() {
        assert_eq!(
            combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
    }
}
