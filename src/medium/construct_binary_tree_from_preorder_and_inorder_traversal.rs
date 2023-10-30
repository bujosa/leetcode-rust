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

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Node {
    todo!("construct_binary_tree_from_preorder_and_inorder_traversal")
}
