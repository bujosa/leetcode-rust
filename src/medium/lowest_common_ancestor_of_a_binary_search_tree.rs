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

    pub fn from_vec(vec: Vec<Option<i32>>) -> Tree {
        let mut nodes: Vec<Tree> = vec![];

        for val in vec {
            match val {
                Some(val) => nodes.push(Some(Rc::new(RefCell::new(TreeNode::new(val))))),
                None => nodes.push(None),
            }
        }

        let mut i = 0;
        let mut j = 1;

        while j < nodes.len() {
            if let Some(node) = nodes[i].as_ref() {
                node.borrow_mut().left = nodes[j].clone();
                node.borrow_mut().right = nodes[j + 1].clone();
            }

            i += 1;
            j += 2;
        }

        nodes[0].clone()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_common_ancestor() {
        let root = TreeNode::from_vec(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);

        let p = TreeNode::from_vec(vec![
            Some(2),
            Some(0),
            Some(4),
            None,
            None,
            Some(3),
            Some(5),
        ]);

        let q = TreeNode::from_vec(vec![Some(8), Some(7), Some(9)]);

        let expected = TreeNode::from_vec(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        assert_eq!(lowest_common_ancestor(root, p, q), expected);
    }
}
