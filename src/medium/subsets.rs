#![allow(dead_code)]
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        assert_eq!(
            subsets(vec![1, 2, 3]),
            vec![
                vec![3],
                vec![1],
                vec![2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2],
                vec![]
            ]
        );
        assert_eq!(subsets(vec![0]), vec![vec![0], vec![]]);
    }
}
