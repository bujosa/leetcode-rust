#![allow(dead_code)]
use std::cell::RefCell;
use std::collections::VecDeque;
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
    if let Some(root) = root {
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, usize)> = VecDeque::from([(root, 0)]);
        let mut res = Vec::with_capacity(100);
        let mut res_len: usize = 0;

        while let Some((node, depth)) = queue.pop_front() {
            let val = node.borrow().val;
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();

            if res_len == depth {
                res.push(val);
                res_len += 1;
            } else {
                res[res_len - 1] = val;
            }

            if let Some(left) = left {
                queue.push_back((left, depth + 1));
            }

            if let Some(right) = right {
                queue.push_back((right, depth + 1));
            }
        }

        res
    } else {
        vec![]
    }
}

/*
    Algorithm - BFS (Breadth First Search) - Level Order Traversal

    1. Create a queue and push the root node and its depth (0) to the queue.
    2. While the queue is not empty, pop the front element of the queue.
    3. If the depth of the popped node is equal to the length of the result vector,
       push the value of the popped node to the result vector.
    4. If the depth of the popped node is not equal to the length of the result vector,
         update the value of the result vector at the index equal to the depth of the popped node.
    5. If the popped node has a left child, push the left child and its depth to the queue.
    6. If the popped node has a right child, push the right child and its depth to the queue.
    7. Return the result vector.

    Time Complexity: O(n)
    Space Complexity: O(n)

*/

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

    #[test]
    fn test_right_side_view_5() {
        let tree = TreeNode::from_vec(vec![Some(1), Some(2), None]);
        assert_eq!(vec![1, 2], right_side_view(tree));
    }
}
