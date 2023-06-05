#![allow(dead_code)]
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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    let mut current = root;
    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            stack.push(node.clone());
            current = node.borrow().left.clone();
        }
        current = stack.pop();
        if let Some(node) = current {
            result.push(node.borrow().val);
            current = node.borrow().right.clone();
        }
    }
    result       
}

/*
    Algorithm:
        1. Create a result vector
        2. Create a stack
        3. Create a current node
        4. While current node is not None or stack is not empty
            1. While current node is not None
                1. Push current node to stack
                2. Set current node to current node's left
            2. Set current node to stack.pop()
            3. If current node is not None
                1. Push current node's value to result vector
                2. Set current node to current node's right
        5. Return result vector

    Time complexity: O(n)
    Space complexity: O(n)
        
 */

#[test]
fn test_inorder_traversal() {
    let mut root = TreeNode::new(1);
    let mut right = TreeNode::new(2);
    let right_left = TreeNode::new(3);
    right.left = Some(Rc::new(RefCell::new(right_left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    assert_eq!(inorder_traversal(Some(Rc::new(RefCell::new(root)))), vec![1, 3, 2]);
}