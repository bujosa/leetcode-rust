#![allow(dead_code)]

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!("Implement it")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validate_permutations(mut result: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) -> bool {
        result.sort();
        result == expected
    }

    #[test]
    fn test_permutations() {
        assert!(validate_permutations(
            permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        ));
    }
}
