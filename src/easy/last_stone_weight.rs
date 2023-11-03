#![allow(dead_code)]
use std::collections::BinaryHeap;

fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut max_heap = BinaryHeap::from(stones);
    while max_heap.len() > 1 {
        let stone1 = max_heap.pop().unwrap();
        let stone2 = max_heap.pop().unwrap();
        if stone1 != stone2 {
            max_heap.push(stone1 - stone2);
        }
    }
    *max_heap.peek().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let stones = vec![2, 7, 4, 1, 8, 1];
        assert_eq!(last_stone_weight(stones), 1);
    }

    #[test]
    fn test_2() {
        let stones = vec![1, 3];
        assert_eq!(last_stone_weight(stones), 2);
    }

    #[test]
    fn test_3() {
        let stones = vec![1];
        assert_eq!(last_stone_weight(stones), 1);
    }
}
