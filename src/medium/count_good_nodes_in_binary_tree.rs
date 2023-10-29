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

    #[inline]
    pub fn from_vec(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

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

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn good_nodes(root: Node) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1448_example_1() {
        let root = TreeNode::from_vec(vec![Some(3), Some(1), Some(4), Some(3), None, Some(1)]);
        let result = 4;

        assert_eq!(good_nodes(root), result);
    }

    #[test]
    fn test_1448_example_2() {
        let root = TreeNode::from_vec(vec![Some(3), Some(3), None, Some(4), Some(2)]);
        let result = 3;

        assert_eq!(good_nodes(root), result);
    }

    #[test]
    fn test_1448_example_3() {
        let root = TreeNode::from_vec(vec![Some(1)]);
        let result = 1;

        assert_eq!(good_nodes(root), result);
    }
}
