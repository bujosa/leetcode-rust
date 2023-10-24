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

pub fn lowest_common_ancestor(root: Tree, p: Tree, q: Tree) -> Tree {
    if root.is_none() {
        return None;
    }

    let root_val = root.as_ref().unwrap().borrow().val;
    let p_val = p.as_ref().unwrap().borrow().val;
    let q_val = q.as_ref().unwrap().borrow().val;

    if p_val > root_val && q_val > root_val {
        lowest_common_ancestor(root.as_ref().unwrap().borrow().right.clone(), p, q)
    } else if p_val < root_val && q_val < root_val {
        lowest_common_ancestor(root.as_ref().unwrap().borrow().left.clone(), p, q)
    } else {
        root
    }
}
