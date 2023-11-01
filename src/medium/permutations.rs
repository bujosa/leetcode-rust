#![allow(dead_code)]

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() == 1 {
        return vec![nums];
    }
    let mut result = vec![];
    for i in 0..nums.len() {
        let mut nums_clone = nums.clone();
        let num = nums_clone.remove(i);
        for mut perm in permute(nums_clone) {
            perm.insert(0, num);
            result.push(perm);
        }
    }
    result
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

    #[test]
    fn test_permutations_2() {
        assert!(validate_permutations(
            permute(vec![0, 1]),
            vec![vec![0, 1], vec![1, 0]]
        ));
    }

    #[test]
    fn test_permutations_3() {
        assert!(validate_permutations(permute(vec![1]), vec![vec![1]]));
    }
}
