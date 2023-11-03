#![allow(dead_code)]

use std::collections::BinaryHeap;
struct KthLargest {
    k: i32,
    heap: BinaryHeap<i32>,
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let heap = BinaryHeap::from(nums.iter().map(|num| -num).collect::<Vec<i32>>());
        let obj = KthLargest {
            heap: heap.clone(),
            k: k,
        };
        return obj;
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(-val);
        while self.heap.len() > self.k as usize {
            self.heap.pop();
        }
        return -self.heap.peek().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_703() {
        let mut obj = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(obj.add(3), 4);
        assert_eq!(obj.add(5), 5);
        assert_eq!(obj.add(10), 5);
        assert_eq!(obj.add(9), 8);
        assert_eq!(obj.add(4), 8);
    }
}
