#![allow(dead_code)]
pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
    let rows = rooms.len();
    let cols = rooms[0].len();
    let mut q = std::collections::VecDeque::new();

    for r in 0..rows {
        for c in 0..cols {
            if rooms[r][c] == 0 {
                q.push_back((r, c));
            }
        }
    }

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut time = 1;

    while !q.is_empty() {
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
                    && rooms[row as usize][col as usize] == i32::MAX
                {
                    rooms[row as usize][col as usize] = time;
                    q.push_back((row as usize, col as usize));
                }
            }
        }
        time += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut rooms = vec![
            vec![i32::MAX, -1, 0, i32::MAX],
            vec![i32::MAX, i32::MAX, i32::MAX, -1],
            vec![i32::MAX, -1, i32::MAX, -1],
            vec![0, -1, i32::MAX, i32::MAX],
        ];
        walls_and_gates(&mut rooms);
        assert_eq!(
            rooms,
            vec![
                vec![3, -1, 0, 1],
                vec![2, 2, 1, -1],
                vec![1, -1, 2, -1],
                vec![0, -1, 3, 4]
            ]
        );
    }
}
