#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

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

    #[inline]
    pub fn from_vec(vec: Vec<Option<i32>>) -> Node {
        let mut vec = vec.into_iter();
        let root = Rc::new(RefCell::new(TreeNode::new(vec.next().unwrap().unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        while let Some(node) = queue.pop_front() {
            if let Some(Some(val)) = vec.next() {
                node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                queue.push_back(node.borrow().left.clone().unwrap());
            }
            if let Some(Some(val)) = vec.next() {
                node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                queue.push_back(node.borrow().right.clone().unwrap());
            }
        }
        Some(root)
    }
}

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut stack = vec![];
    let mut node = root;
    let mut k = k;
    while node.is_some() || !stack.is_empty() {
        while let Some(n) = node {
            stack.push(n.clone());
            node = n.borrow().left.clone();
        }
        node = stack.pop();
        k -= 1;
        if k == 0 {
            return node.unwrap().borrow().val;
        }
        node = node.unwrap().borrow().right.clone();
    }
    0
}

/*
    Algorithm - Kth Smallest Element in a BST
    1. Create a stack
    2. Create a node and set it to root
    3. While node is not None or stack is not empty
        1. While node is not None
            1. Push node to stack
            2. Set node to node's left
        2. Set node to stack.pop()
        3. Decrement k
        4. If k is 0
            1. Return node's value
        5. Set node to node's right

    Analysis
        Time Complexity: O(n)
        Space Complexity: O(n)

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_230() {
        assert_eq!(
            kth_smallest(
                TreeNode::from_vec(vec![Some(3), Some(1), Some(4), None, Some(2)]),
                1
            ),
            1
        );
        assert_eq!(
            kth_smallest(
                TreeNode::from_vec(vec![
                    Some(5),
                    Some(3),
                    Some(6),
                    Some(2),
                    Some(4),
                    None,
                    None,
                    Some(1)
                ]),
                3
            ),
            3
        );
    }
}
