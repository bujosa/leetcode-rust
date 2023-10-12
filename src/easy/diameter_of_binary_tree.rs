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

pub fn diameter_of_binary_tree(root: Tree) -> i32 {
    let mut max = 0;
    fn helper(root: Tree, max: &mut i32) -> i32 {
        if let Some(node) = root {
            let left = helper(node.borrow().left.clone(), max);
            let right = helper(node.borrow().right.clone(), max);
            *max = std::cmp::max(*max, left + right);
            std::cmp::max(left, right) + 1
        } else {
            0
        }
    }
    helper(root, &mut max);
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut root = TreeNode::new(1);
        let mut left = TreeNode::new(2);
        let right = TreeNode::new(3);
        let left_left = TreeNode::new(4);
        let left_right = TreeNode::new(5);

        left.left = Some(Rc::new(RefCell::new(left_left)));
        left.right = Some(Rc::new(RefCell::new(left_right)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(
            diameter_of_binary_tree(Some(Rc::new(RefCell::new(root)))),
            3
        );
    }

    #[test]
    fn test_2() {
        let mut root = TreeNode::new(1);
        let mut left = TreeNode::new(2);
        let mut right = TreeNode::new(3);

        left.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        left.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

        right.left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        right.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(
            diameter_of_binary_tree(Some(Rc::new(RefCell::new(root)))),
            4
        );
    }
}
