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

/*
    Algorithm - BFS (Breadth First Search) - Iterative

    1. Create a queue and push the root node into it
    2. While the queue is not empty, create a level array and a next queue
    3. Iterate through the queue and push the node's value into the level array
    4. If the node has a left child, push it into the next queue
    5. If the node has a right child, push it into the next queue
    6. Push the level array into the result array
    7. Set the queue to the next queue
    8. Return the result array

    Complexity Analysis
     - Time Complexity: O(n)
     - Space Complexity: O(n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_order() {
        let mut t1 = TreeNode::new(3);
        let mut t2 = TreeNode::new(9);
        let t3 = TreeNode::new(20);
        let t4 = TreeNode::new(15);
        let t5 = TreeNode::new(7);
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));
        let result = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(level_order(Some(Rc::new(RefCell::new(t1)))), result);
    }
}
