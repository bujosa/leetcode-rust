#![allow(dead_code)]
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = vec![vec![false; 9]; 9];
    let mut cols = vec![vec![false; 9]; 9];
    let mut boxes = vec![vec![false; 9]; 9];

    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == '.' {
                continue;
            }
            let num = board[i][j] as usize - '1' as usize;
            if rows[i][num] || cols[j][num] || boxes[(i / 3) * 3 + j / 3][num] {
                return false;
            }
            rows[i][num] = true;
            cols[j][num] = true;
            boxes[(i / 3) * 3 + j / 3][num] = true;
        }
    }
    true
}

/*
   Algorithm - Array
   - Iterate through each cell in the board
   - If the cell is not empty, check if the value is in the row, column, or box
   - If the value is in the row, column, or box, return false
   - If the value is not in the row, column, or box, add the value to the row, column, and box
   - Return true

   Time: O(1) - The board is always 9x9
   Space: O(1) - The board is always 9x9
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_sudoku() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(board), true);
    }

    #[test]
    fn test_invalid_sodoku() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(board), false);
    }
}
