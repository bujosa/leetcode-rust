#![allow(dead_code)]
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    todo!("Implement solve_n_queens()")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_51() {
        assert_eq!(
            solve_n_queens(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
    }
}
