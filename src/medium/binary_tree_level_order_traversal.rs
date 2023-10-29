#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    if root.is_none() {
        return result;
    }
    let mut queue = vec![root.unwrap()];
    while !queue.is_empty() {
        let mut level = vec![];
        let mut next_queue = vec![];
        for node in queue {
            level.push(node.borrow().val);
            if node.borrow().left.is_some() {
                next_queue.push(node.borrow().left.clone().unwrap());
            }
            if node.borrow().right.is_some() {
                next_queue.push(node.borrow().right.clone().unwrap());
            }
        }
        result.push(level);
        queue = next_queue;
    }
    result
}
