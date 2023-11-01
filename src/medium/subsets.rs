#![allow(dead_code)]
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![]];
    for num in nums {
        let temp = result.clone();
        for mut item in temp {
            item.push(num);
            result.push(item);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        assert_eq!(
            subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }
}
