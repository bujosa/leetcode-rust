#![allow(dead_code)]
pub fn oranges_rotting(grid: &mut Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut fresh = 0;
    let mut time = 0;
    let mut q = std::collections::VecDeque::new();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 1 {
                fresh += 1;
            }
            if grid[r][c] == 2 {
                q.push_back((r, c));
            }
        }
    }

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while !q.is_empty() && fresh > 0 {
        let qlen = q.len();
        for _ in 0..qlen {
            let (r, c) = q.pop_front().unwrap();

            for (dr, dc) in &directions {
                let row = r as i32 + dr;
                let col = c as i32 + dc;
                if row >= 0
                    && col >= 0
                    && row < rows as i32
                    && col < cols as i32
                    && grid[row as usize][col as usize] == 1
                {
                    grid[row as usize][col as usize] = 2;
                    q.push_back((row as usize, col as usize));
                    fresh -= 1;
                }
            }
        }
        time += 1;
    }

    if fresh == 0 {
        time
    } else {
        -1
    }
}

#[test]
fn test_oranges_rotting() {
    let mut grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    assert_eq!(oranges_rotting(&mut grid), 4);
    let mut grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
    assert_eq!(oranges_rotting(&mut grid), -1);
    let mut grid = vec![vec![0, 2]];
    assert_eq!(oranges_rotting(&mut grid), 0);
}
