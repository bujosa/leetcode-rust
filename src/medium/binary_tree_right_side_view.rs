#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
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
        let mut nodes: Vec<Node> = vec![];

        for val in vec {
            nodes.push(match val {
                Some(val) => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
                None => None,
            });
        }

        for i in 0..nodes.len() {
            if let Some(node) = &nodes[i] {
                let left = 2 * i + 1;
                let right = 2 * i + 2;
                if left < nodes.len() {
                    node.borrow_mut().left = nodes[left].clone();
                }
                if right < nodes.len() {
                    node.borrow_mut().right = nodes[right].clone();
                }
            }
        }

        nodes[0].clone()
    }
}

pub fn right_side_view(root: Node) -> Vec<i32> {}
