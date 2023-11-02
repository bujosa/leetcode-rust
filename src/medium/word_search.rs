#![allow(dead_code)]
fn dfs(
    board: &[Vec<char>],
    word: &[char],
    visited: &mut [Vec<bool>],
    m: usize,
    n: usize,
    len: usize,
    i: usize,
    j: usize,
    depth: usize,
) -> bool {
    depth == len
        || (i < m && j < n && !visited[i][j] && word[depth] == board[i][j] && {
            visited[i][j] = true;
            let rez = [0, 1, 0, !0, 0].windows(2).any(|w| {
                dfs(
                    board,
                    word,
                    visited,
                    m,
                    n,
                    len,
                    i.wrapping_add(w[0]),
                    j.wrapping_add(w[1]),
                    depth + 1,
                )
            });
            visited[i][j] = false;
            rez
        })
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let m = board.len();
    let n = board[0].len();
    let word = word.chars().collect::<Vec<_>>();
    let len = word.len();
    let mut visited = vec![vec![false; n]; m];

    (0..m).any(|i| (0..n).any(|j| dfs(&board, &word, &mut visited, m, n, len, i, j, 0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exist() {
        assert_eq!(
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_string()
            ),
            true
        );
        assert_eq!(
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "SEE".to_string()
            ),
            true
        );
        assert_eq!(
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCB".to_string()
            ),
            false
        );
    }
}
