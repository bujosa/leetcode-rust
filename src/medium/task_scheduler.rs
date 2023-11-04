#![allow(dead_code)]
use std::collections::{BinaryHeap, VecDeque};

fn least_interval(tasks: Vec<char>, n: usize) -> usize {
    let mut heap = BinaryHeap::new();
    let mut q = VecDeque::new();
    let mut map = vec![0; 26];

    for t in tasks {
        map[(t as u8 - 'A' as u8) as usize] += 1;
    }

    for (i, cnt) in map.into_iter().enumerate() {
        if cnt > 0 {
            let c = ('A' as u8 + i as u8) as char;
            heap.push((cnt, c));
        }
    }

    let mut time = 0;

    while !heap.is_empty() || !q.is_empty() {
        while let Some(item) = q.pop_front() {
            let (process_time, cnt, c) = item;

            if process_time < time {
                heap.push((cnt, c));
            } else {
                q.push_front(item);
                break;
            }
        }

        if let Some((cnt, c)) = heap.pop() {
            if cnt > 1 {
                q.push_back((time + n, cnt - 1, c));
            }
        }

        time += 1;
    }

    time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        let n = 2;
        assert_eq!(least_interval(tasks, n), 8);
    }

    #[test]
    fn test_2() {
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        let n = 0;
        assert_eq!(least_interval(tasks, n), 6);
    }

    #[test]
    fn test_3() {
        let tasks = vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'];
        let n = 2;
        assert_eq!(least_interval(tasks, n), 16);
    }
}
