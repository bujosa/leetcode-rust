#![allow(dead_code)]
pub fn solve(board: &mut Vec<Vec<char>>) {
    let rows = board.len();
    let cols = board[0].len();

    fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize) {
        let rows = board.len();
        let cols = board[0].len();

        if r >= rows || c >= cols || board[r][c] != 'O' {
            return;
        }

        board[r][c] = '#';

        if r > 0 {
            dfs(board, r - 1, c);
        }
        if c > 0 {
            dfs(board, r, c - 1);
        }
        dfs(board, r + 1, c);
        dfs(board, r, c + 1);
    }

    for r in 0..rows {
        dfs(board, r, 0);
        dfs(board, r, cols - 1);
    }

    for c in 0..cols {
        dfs(board, 0, c);
        dfs(board, rows - 1, c);
    }

    for r in 0..rows {
        for c in 0..cols {
            if board[r][c] == '#' {
                board[r][c] = 'O';
            } else if board[r][c] == 'O' {
                board[r][c] = 'X';
            }
        }
    }
}
