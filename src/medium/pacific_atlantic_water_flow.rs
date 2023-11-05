#![allow(dead_code)]
pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    if heights.is_empty() {
        return res;
    }

    let rows = heights.len();
    let cols = heights[0].len();
    let mut pac = vec![vec![false; cols]; rows];
    let mut atl = vec![vec![false; cols]; rows];

    for r in 0..rows {
        dfs(&heights, r, 0, i32::MIN, &mut pac);
        dfs(&heights, r, cols - 1, i32::MIN, &mut atl);
    }

    for c in 0..cols {
        dfs(&heights, 0, c, i32::MIN, &mut pac);
        dfs(&heights, rows - 1, c, i32::MIN, &mut atl);
    }

    for r in 0..rows {
        for c in 0..cols {
            if pac[r][c] && atl[r][c] {
                res.push(vec![r as i32, c as i32]);
            }
        }
    }

    res
}

fn dfs(heights: &Vec<Vec<i32>>, r: usize, c: usize, pre_height: i32, ocean: &mut Vec<Vec<bool>>) {
    if r == heights.len() || c == heights[0].len() || ocean[r][c] || heights[r][c] < pre_height {
        return;
    }

    ocean[r][c] = true;
    dfs(heights, r + 1, c, heights[r][c], ocean);
    if r > 0 {
        dfs(heights, r - 1, c, heights[r][c], ocean);
    }
    dfs(heights, r, c + 1, heights[r][c], ocean);
    if c > 0 {
        dfs(heights, r, c - 1, heights[r][c], ocean);
    }
}

#[test]
fn test_pacific_atlantic() {
    let heights = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    let res = vec![
        vec![0, 4],
        vec![1, 3],
        vec![1, 4],
        vec![2, 2],
        vec![3, 0],
        vec![3, 1],
        vec![4, 0],
    ];
    assert_eq!(pacific_atlantic(heights), res);
}
