#![allow(dead_code)]
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node { val, next: None }
    }

    pub fn from_vec(vec: Vec<i32>) -> Option<Box<Node>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = Node::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<Node>>>) -> Option<Box<Node>> {
    let mut heap = BinaryHeap::new();
    for list in lists {
        if let Some(node) = list {
            heap.push(node); // O(logk)
        }
    }

    let mut res = None;
    let mut res_tail = &mut res;
    while let Some(mut node) = heap.pop() {
        if let Some(next) = node.next.take() {
            heap.push(next); // O(logk)
        }
        *res_tail = Some(node);
        res_tail = &mut res_tail.as_mut().unwrap().next;
    }
    res
}

/*
    Algorithm - O(nlogk)

    1. Create a min heap of size k
    2. Insert the first element of each list into the heap
    3. Pop the min element from the heap and insert the next element from the list of the popped element
    4. Repeat step 3 until all the lists are empty

    Time Complexity - O(nlogk)
    Space Complexity - O(k)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_k_lists_test() {
        let lists = vec![
            Node::from_vec(vec![1, 4, 5]),
            Node::from_vec(vec![1, 3, 4]),
            Node::from_vec(vec![2, 6]),
        ];
        let res = merge_k_lists(lists);
        let expected = Node::from_vec(vec![1, 1, 2, 3, 4, 4, 5, 6]);
        assert_eq!(res, expected);
    }

    #[test]
    fn merge_k_lists_test_2() {
        let lists = vec![Node::from_vec(vec![]), Node::from_vec(vec![])];
        let res = merge_k_lists(lists);
        let expected = Node::from_vec(vec![]);
        assert_eq!(res, expected);
    }

    #[test]
    fn merge_k_lists_test_3() {
        let lists = vec![];
        let res = merge_k_lists(lists);
        let expected = Node::from_vec(vec![]);
        assert_eq!(res, expected);
    }
}

// Solution without using BinaryHeap
// pub fn merge_k_lists(lists: Vec<Option<Box<Node>>>) -> Option<Box<Node>> {
//     let mut lists = lists;
//     let mut res = None;
//     let mut res_tail = &mut res;
//     loop {
//         let mut min = std::i32::MAX;
//         let mut min_idx = 0;
//         let mut all_none = true;
//         for (i, list) in lists.iter().enumerate() {
//             if let Some(node) = list {
//                 all_none = false;
//                 if node.val < min {
//                     min = node.val;
//                     min_idx = i;
//                 }
//             }
//         }
//         if all_none {
//             break;
//         }
//         let mut node = lists[min_idx].take().unwrap();
//         lists[min_idx] = node.next.take();
//         *res_tail = Some(node);
//         res_tail = &mut res_tail.as_mut().unwrap().next;
//     }
//     res
// }
// Algorithm - O(nk)
// Time Complexity - O(nk)
// Space Complexity - O(1)
