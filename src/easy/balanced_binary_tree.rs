#![allow(dead_code)]
use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }

    let left_height = height(root.as_ref().unwrap().borrow().left.clone());
    let right_height = height(root.as_ref().unwrap().borrow().right.clone());

    if (left_height - right_height).abs() > 1 {
        return false;
    }

    return is_balanced(root.as_ref().unwrap().borrow().left.clone()) && is_balanced(root.as_ref().unwrap().borrow().right.clone());
}

fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let left_height = height(root.as_ref().unwrap().borrow().left.clone());
    let right_height = height(root.as_ref().unwrap().borrow().right.clone());

    return 1 + std::cmp::max(left_height, right_height);
}

/*
    Algorithm Name: Recursive DFS (Depth First Search)
        1. If the root is None, return true
        2. Get the height of the left subtree
        3. Get the height of the right subtree
        4. If the difference between the left and right subtree is greater than 1, return false
        5. Recursively call is_balanced on the left and right subtree
        6. Return the result of the recursive calls

    Time complexity: O(n)
    Space complexity: O(n)
 */


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_110() {
        let mut t1 = TreeNode::new(3);
        let mut t2 = TreeNode::new(9);
        let t3 = TreeNode::new(20);
        let t4 = TreeNode::new(15);
        let t5 = TreeNode::new(7);
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));
        assert_eq!(is_balanced(Some(Rc::new(RefCell::new(t1)))), true);

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(2);
        let mut t4 = TreeNode::new(3);
        let mut t5 = TreeNode::new(3);
        let mut t6 = TreeNode::new(4);
        let t7 = TreeNode::new(4);
        t6.left = Some(Rc::new(RefCell::new(t7)));
        t5.left = Some(Rc::new(RefCell::new(t6)));
        t4.left = Some(Rc::new(RefCell::new(t5)));
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t2.left = Some(Rc::new(RefCell::new(t3)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        assert_eq!(is_balanced(Some(Rc::new(RefCell::new(t1)))), false);
    }
}
