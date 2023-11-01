#![allow(dead_code)]

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    todo!("Implement this function")
}

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
