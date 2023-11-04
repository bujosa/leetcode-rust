#![allow(dead_code)]
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);
        if let Some(max_heap_top) = self.max_heap.pop() {
            self.min_heap.push(Reverse(max_heap_top));
        }
        if self.max_heap.len() < self.min_heap.len() {
            if let Some(Reverse(min_heap_top)) = self.min_heap.pop() {
                self.max_heap.push(min_heap_top);
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.max_heap.len() > self.min_heap.len() {
            *self.max_heap.peek().unwrap() as f64
        } else {
            (*self.max_heap.peek().unwrap() as f64 + self.min_heap.peek().unwrap().0 as f64) / 2.0
        }
    }
}

#[test]
fn test_find_median_from_data_stream() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(1);
    median_finder.add_num(2);
    assert_eq!(median_finder.find_median(), 1.5);
    median_finder.add_num(3);
    assert_eq!(median_finder.find_median(), 2.0);
}
