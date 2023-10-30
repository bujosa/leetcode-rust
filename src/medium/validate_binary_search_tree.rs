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

fn is_valid_bst(root: Node) -> bool {
    fn helper(node: Node, min: Option<i32>, max: Option<i32>) -> bool {
        if let Some(n) = node {
            let val = n.borrow().val;
            if let Some(min) = min {
                if val <= min {
                    return false;
                }
            }
            if let Some(max) = max {
                if val >= max {
                    return false;
                }
            }
            return helper(n.borrow().left.clone(), min, Some(val))
                && helper(n.borrow().right.clone(), Some(val), max);
        }
        true
    }

    helper(root, None, None)
}

/*
    Algorithm - Recursion (DFS)

    - Check if the current node's value is between min and max
    - Check if the left subtree is valid
    - Check if the right subtree is valid
    - If all the above conditions are met, return true
    - If any of the above conditions are not met, return false

    Complexity:
        Time: O(n)
        Space: O(n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_98() {
        assert_eq!(
            is_valid_bst(TreeNode::from_vec(vec![Some(2), Some(1), Some(3)])),
            true
        );
        assert_eq!(
            is_valid_bst(TreeNode::from_vec(vec![
                Some(5),
                Some(1),
                Some(4),
                None,
                None,
                Some(3),
                Some(6)
            ])),
            false
        );
        assert_eq!(
            is_valid_bst(TreeNode::from_vec(vec![
                Some(10),
                Some(5),
                Some(15),
                None,
                None,
                Some(6),
                Some(20)
            ])),
            false
        );
    }
}
