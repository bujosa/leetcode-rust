#![allow(dead_code)]
pub fn max_area_of_island(grid: &mut Vec<Vec<i32>>) -> i32 {
    let mut max_area = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        if i >= rows || j >= cols || grid[i][j] == 0 {
            return 0;
        }

        grid[i][j] = 0;
        let mut area = 1;

        if i > 0 {
            area += dfs(grid, i - 1, j);
        }
        if j > 0 {
            area += dfs(grid, i, j - 1);
        }
        area += dfs(grid, i + 1, j);
        area += dfs(grid, i, j + 1);

        area
    }

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != 0 {
                max_area = max_area.max(dfs(grid, i, j));
            }
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area_of_island() {
        assert_eq!(
            max_area_of_island(&mut vec![vec![0, 0, 0, 0, 0, 0, 0, 0]]),
            0
        );
        assert_eq!(
            max_area_of_island(&mut vec![
                vec![0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0]
            ]),
            2
        );
        assert_eq!(
            max_area_of_island(&mut vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            ]),
            6
        );
    }
}
