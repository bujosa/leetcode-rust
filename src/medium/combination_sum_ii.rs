#![allow(dead_code)]

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();
    fn backtracking(
        target: i32,
        start: usize,
        candidates: &Vec<i32>,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target < 0 {
            return;
        }
        if target == 0 {
            result.push(path.clone());
            return;
        }
        for i in start..candidates.len() {
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }
            path.push(candidates[i]);
            backtracking(target - candidates[i], i + 1, candidates, path, result);
            path.pop();
        }
    }

    let mut result = Vec::new();
    backtracking(target, 0, &candidates, &mut Vec::new(), &mut result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum_ii() {
        assert_eq!(
            combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }
}
