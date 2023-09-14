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
}

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if root.is_none() {
        return false;
    }

    let mut stack = vec![(root, target_sum)];

    while !stack.is_empty() {
        let (node, sum) = stack.pop().unwrap();
        let node = node.unwrap();
        let node = node.borrow();
        if node.left.is_none() && node.right.is_none() && node.val == sum {
            return true;
        }
        if node.left.is_some() {
            stack.push((node.left.clone(), sum - node.val));
        }
        if node.right.is_some() {
            stack.push((node.right.clone(), sum - node.val));
        }
    }
    false
}

/*
 Algorithm - DFS
 - If root is None, return false
 - If root is not None, check if root is leaf node and if root.val == target_sum
 - If root is not leaf node, recursively call has_path_sum on left and right child
 - Return true if either of the recursive call returns true
 - Return false if both recursive call returns false

 Complexity
 - Time is O(n) where n is the number of nodes in the tree
 - Space is O(n) where n is the number of nodes in the tree

*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_path_sum() {
        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(4);
        let mut t3 = TreeNode::new(8);
        let mut t4 = TreeNode::new(11);
        let mut t6 = TreeNode::new(4);
        let t7 = TreeNode::new(7);
        let t8 = TreeNode::new(2);
        let t9 = TreeNode::new(1);
        t6.right = Some(Rc::new(RefCell::new(t9)));
        t4.right = Some(Rc::new(RefCell::new(t8)));
        t4.left = Some(Rc::new(RefCell::new(t7)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.right = Some(Rc::new(RefCell::new(t3)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        assert_eq!(has_path_sum(Some(Rc::new(RefCell::new(t1))), 22), true);
    }
}
