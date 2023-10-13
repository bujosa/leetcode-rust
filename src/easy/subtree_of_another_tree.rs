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

pub fn is_subtree(root: Tree, sub_root: Tree) -> bool {
    todo!()
}

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
        let root_right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let root_left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let root_left_left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let sub_root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let sub_root_left = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        root_left.as_ref().unwrap().borrow_mut().left = root_left_left;
        root.as_ref().unwrap().borrow_mut().left = root_left;
        root.as_ref().unwrap().borrow_mut().right = root_right;
        sub_root.as_ref().unwrap().borrow_mut().left = sub_root_left;
        assert_eq!(is_subtree(root, sub_root), true);
    }
}
