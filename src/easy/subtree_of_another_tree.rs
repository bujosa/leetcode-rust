#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn add_left(&mut self, val: i32) {
        let left_node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        self.left = left_node;
    }

    pub fn add_right(&mut self, val: i32) {
        let right_node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        self.right = right_node;
    }
}

pub fn is_subtree(root: Tree, sub_root: Tree) -> bool {
    match (root, sub_root) {
        (None, _) => false,
        (Some(root), Some(sub_root)) => {
            is_same_tree(Some(root.clone()), Some(sub_root.clone()))
                || is_subtree(root.borrow().left.clone(), Some(sub_root.clone()))
                || is_subtree(root.borrow().right.clone(), Some(sub_root.clone()))
        }
        _ => false,
    }
}

/*
    Algorithm - Recursion

    - If root is None, then return false
    - If root is not None, then check if root and sub_root are same
        - If they are same, then return true
        - If they are not same, then check if left subtree of root and sub_root are same
            - If they are same, then return true
            - If they are not same, then check if right subtree of root and sub_root are same
                - If they are same, then return true
                - If they are not same, then return false
*/

fn is_same_tree(root: Tree, sub_root: Tree) -> bool {
    match (root, sub_root) {
        (None, None) => true,
        (Some(root), Some(sub_root)) => {
            root.borrow().val == sub_root.borrow().val
                && is_same_tree(root.borrow().left.clone(), sub_root.borrow().left.clone())
                && is_same_tree(root.borrow().right.clone(), sub_root.borrow().right.clone())
        }
        _ => false,
    }
}

/*
    Algorithm - Recursion
     - If both trees are empty then they are same
     - If both trees are non-empty
         - Check if current data of both trees are same
         - Recursively check if left subtree of both trees are same
         - Recursively check if right subtree of both trees are same
     - If one of them is empty and other is not, then they are not same
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_not_subtree() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let sub_root = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        assert_eq!(is_subtree(root, sub_root), false);
    }

    #[test]
    fn test_is_subtree() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let sub_root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        assert_eq!(is_subtree(root, sub_root), true);
    }

    #[test]
    fn test_is_subtree_large() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.as_ref().unwrap().borrow_mut().add_left(4);
        root.as_ref().unwrap().borrow_mut().add_right(5);
        root.as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .add_left(1);
        let sub_root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        sub_root.as_ref().unwrap().borrow_mut().add_left(1);
        assert_eq!(is_subtree(root, sub_root), true);
    }
}
