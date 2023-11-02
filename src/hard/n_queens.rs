#![allow(dead_code)]
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut board = vec![vec!['.'; n as usize]; n as usize];
    let mut res = Vec::new();
    let mut posdiag = vec![false; 2 * n as usize - 1];
    let mut negdiag = vec![false; 2 * n as usize - 1];
    let mut cols = vec![false; n as usize];
    backtrack(
        0,
        &mut board,
        &mut res,
        &mut posdiag,
        &mut negdiag,
        &mut cols,
    );
    res
}

fn backtrack(
    row: i32,
    board: &mut Vec<Vec<char>>,
    res: &mut Vec<Vec<String>>,
    posdiag: &mut Vec<bool>,
    negdiag: &mut Vec<bool>,
    cols: &mut Vec<bool>,
) {
    let n = board.len() as i32;
    if row == n {
        res.push(board.iter().map(|r| r.iter().collect()).collect());
        return;
    }
    for col in 0..n {
        if posdiag[(row + col) as usize]
            || negdiag[(row - col + n - 1) as usize]
            || cols[col as usize]
        {
            continue;
        }

        posdiag[(row + col) as usize] = true;
        negdiag[(row - col + n - 1) as usize] = true;
        cols[col as usize] = true;
        board[row as usize][col as usize] = 'Q';
        backtrack(row + 1, board, res, posdiag, negdiag, cols);
        posdiag[(row + col) as usize] = false;
        negdiag[(row - col + n - 1) as usize] = false;
        cols[col as usize] = false;
        board[row as usize][col as usize] = '.';
    }
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
