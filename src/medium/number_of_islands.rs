#![allow(dead_code)]
pub fn num_islands(grid: &mut Vec<Vec<char>>) -> i32 {
    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut islands = 0;

    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let rows = grid.len();
        let cols = grid[0].len();

        if i >= rows || j >= cols || grid[i][j] == '0' {
            return;
        }

        grid[i][j] = '0';

        if i > 0 {
            dfs(grid, i - 1, j);
        }
        if j > 0 {
            dfs(grid, i, j - 1);
        }
        dfs(grid, i + 1, j);
        dfs(grid, i, j + 1);
    }

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '1' {
                islands += 1;
                dfs(grid, i, j);
            }
        }
    }

    islands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_islands() {
        assert_eq!(
            num_islands(&mut vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
        assert_eq!(
            num_islands(&mut vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
