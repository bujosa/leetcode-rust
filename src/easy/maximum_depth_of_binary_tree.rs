#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

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
}

pub fn max_depth(root: Tree) -> i32 {
    if root.is_none() {
        return 0;
    }

    let left_depth = max_depth(root.as_ref().unwrap().borrow().left.clone());
    let right_depth = max_depth(root.as_ref().unwrap().borrow().right.clone());

    return 1 + std::cmp::max(left_depth, right_depth);
}

/*
   Algorithm - Recursive DFS (Depth First Search)
    - If the root is None, return 0
     - Get the max depth of the left subtree
     - Get the max depth of the right subtree
     - Return 1 + the max of the left and right subtree

   Complexity
       - Time: O(n)
       - Space: O(n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut root = TreeNode::new(3);
        let mut left = TreeNode::new(9);
        let right = TreeNode::new(20);
        let left_left = TreeNode::new(15);
        let left_right = TreeNode::new(7);

        left.left = Some(Rc::new(RefCell::new(left_left)));
        left.right = Some(Rc::new(RefCell::new(left_right)));
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(max_depth(Some(Rc::new(RefCell::new(root)))), 3);
    }

    #[test]
    fn test_2() {
        let mut root = TreeNode::new(1);
        let left = TreeNode::new(2);

        root.left = Some(Rc::new(RefCell::new(left)));

        assert_eq!(max_depth(Some(Rc::new(RefCell::new(root)))), 2);
    }

    #[test]
    fn test_3() {
        let root = TreeNode::new(0);

        assert_eq!(max_depth(Some(Rc::new(RefCell::new(root)))), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(max_depth(None), 0);
    }
}
