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

pub fn right_side_view(root: Node) -> Vec<i32> {
    let mut result = vec![];
    let mut queue = vec![];
    if let Some(node) = root {
        queue.push(node);
    }

    while !queue.is_empty() {
        if let Some(node) = queue.pop() {
            result.push(node.borrow().val);
            if let Some(right) = node.borrow_mut().right.take() {
                queue.push(right);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_side_view() {
        let tree = TreeNode::from_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            Some(5),
            None,
            Some(4),
        ]);
        assert_eq!(vec![1, 3, 4], right_side_view(tree));
    }

    #[test]
    fn test_right_side_view_2() {
        let tree = TreeNode::from_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            Some(5),
        ]);
        assert_eq!(vec![1, 3, 5], right_side_view(tree));
    }

    #[test]
    fn test_right_side_view_3() {
        let tree = TreeNode::from_vec(vec![Some(1), None, Some(3)]);
        assert_eq!(vec![1, 3], right_side_view(tree));
    }

    #[test]
    fn test_right_side_view_4() {
        let expected = vec![] as Vec<i32>;
        assert_eq!(expected, right_side_view(None));
    }
}
