#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
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

    #[inline]
    pub fn from_vec(vec: Vec<Option<i32>>) -> Tree {
        let mut nodes = vec![];

        for val in vec {
            match val {
                Some(val) => nodes.push(Some(Rc::new(RefCell::new(TreeNode::new(val))))),
                None => nodes.push(None),
            }
        }

        let mut i = 0;
        while i < nodes.len() {
            if let Some(node) = nodes[i].as_ref() {
                let left = i * 2 + 1;
                if left < nodes.len() {
                    node.borrow_mut().left = nodes[left].clone();
                }
                let right = i * 2 + 2;
                if right < nodes.len() {
                    node.borrow_mut().right = nodes[right].clone();
                }
            }
            i += 1;
        }

        nodes[0].clone()
    }
}

pub fn level_order(root: Tree) -> Vec<Vec<i32>> {
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
        let t = TreeNode::from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let result = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(level_order(t), result);
    }
}
