#![allow(dead_code)]
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    for i in 0..num_rows as usize {
        let mut row = Vec::new();
        for j in 0..=i {
            if j == 0 || j == i {
                row.push(1);
            } else {
                row.push(result[i - 1][j - 1] + result[i - 1][j]);
            }
        }
        result.push(row);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let result = generate(5);
        assert_eq!(
            result,
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
