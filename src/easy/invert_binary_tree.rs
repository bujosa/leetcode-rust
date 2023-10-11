#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

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

type Tree = Option<Rc<RefCell<TreeNode>>>;

pub fn invert_tree(root: Tree) -> Tree {
    root.map(|root| {
        match root.borrow_mut() {
            mut node => {
                let left = node.left.clone();
                let right = node.right.clone();
                node.left = invert_tree(right);
                node.right = invert_tree(left);
            }
        };
        root
    })
}

/*
    Algorithm - Recursive

    1. If root is None, return None
    2. Swap the left and right nodes
    3. Recursively call invert_tree on the left and right nodes
    4. Return the root

    Time: O(n)
    Space: O(n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_226() {
        assert_eq!(
            invert_tree(Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            })))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            })))
        );
    }
}
