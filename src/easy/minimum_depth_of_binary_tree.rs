#![allow(dead_code)]
use std::rc::Rc;
use std::cell::RefCell;


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


pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut min = std::i32::MAX;
    let mut stack = vec![(root, 1)];
    
    while !stack.is_empty() {
        let (node, depth) = stack.pop().unwrap();
        let node = node.unwrap();
        let node = node.borrow();
        if node.left.is_none() && node.right.is_none() {
            min = min.min(depth);
        }
        if node.left.is_some() {
            stack.push((node.left.clone(), depth + 1));
        }
        if node.right.is_some() {
            stack.push((node.right.clone(), depth + 1));
        }
    }
    min
}

/*
    Algorithm:
        - If root is None, return 0
        - If root.left is None and root.right is None, return 1
        - If root.left is None, return 1 + min_depth(root.right)
        - If root.right is None, return 1 + min_depth(root.left)
        - Return 1 + min(min_depth(root.left), min_depth(root.right))
    
    Time: O(n)
    Space: O(n)
 */

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_min_depth() {
        let mut root = TreeNode::new(3);
        let mut left = TreeNode::new(9);
        let mut right = TreeNode::new(20);
        let right_left = TreeNode::new(15);
        let right_right = TreeNode::new(7);
        right.left = Some(Rc::new(RefCell::new(right_left)));
        right.right = Some(Rc::new(RefCell::new(right_right)));
        left.left = None;
        left.right = None;
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));
        assert_eq!(min_depth(Some(Rc::new(RefCell::new(root)))), 2);
    }
}
