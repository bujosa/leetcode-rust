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
    fn helper(preorder: &[i32], inorder: &[i32]) -> Node {
        if preorder.is_empty() {
            return None;
        }
        let root = preorder[0];
        let mut root_idx = 0;
        for (i, &val) in inorder.iter().enumerate() {
            if val == root {
                root_idx = i;
                break;
            }
        }
        let mut node = TreeNode::new(root);
        node.left = helper(&preorder[1..=root_idx], &inorder[..root_idx]);
        node.right = helper(&preorder[root_idx + 1..], &inorder[root_idx + 1..]);
        Some(Rc::new(RefCell::new(node)))
    }
    helper(&preorder, &inorder)
}

/*
    Algorithm - Recursion

    Preorder: [root, left, right]
    Inorder: [left, root, right]

    1. The first element in preorder is the root.
    2. Find the root in inorder, the elements on the left of the root are the left subtree, the elements on the right of the root are the right subtree.
    3. Recursively build the left and right subtree.
    4. Return the root.

    Time complexity: O(n)
    Space complexity: O(n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tree() {
        assert_eq!(
            build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            TreeNode::from_vec(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])
        );
        assert_eq!(
            build_tree(vec![-1], vec![-1]),
            TreeNode::from_vec(vec![Some(-1)])
        );
    }
}
