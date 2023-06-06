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

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }

    let mut queue = std::collections::VecDeque::new();
    
    queue.push_back(root.clone());
    queue.push_back(root.clone());
    
    while !queue.is_empty() {
        let t1 = queue.pop_front().unwrap();
        let t2 = queue.pop_front().unwrap();
        if t1.is_none() && t2.is_none() {
            continue;
        }
        if t1.is_none() || t2.is_none() {
            return false;
        }
        let t1 = t1.unwrap();
        let t2 = t2.unwrap();
        let t1 = t1.borrow();
        let t2 = t2.borrow();
        if t1.val != t2.val {
            return false;
        }
        queue.push_back(t1.left.clone());
        queue.push_back(t2.right.clone());
        queue.push_back(t1.right.clone());
        queue.push_back(t2.left.clone());
    }
    true
        
}

/* 
    Algorithm - BFS

    1. Push root node twice into queue
    2. Pop two nodes from queue
    3. If both nodes are None, continue
    4. If one of the nodes is None, return false
    5. If both nodes are not None, check if their values are equal
    6. Push left child of first node and right child of second node into queue
    7. Push right child of first node and left child of second node into queue
    8. Repeat steps 2-7 until queue is empty

    Time: O(n)
    Space: O(n)
    
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_101() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(2);
        let  t4 = TreeNode::new(3);
        let  t5 = TreeNode::new(4);
        let  t6 = TreeNode::new(4);
        let  t7 = TreeNode::new(3);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));
        assert_eq!(is_symmetric(Some(Rc::new(RefCell::new(t1)))), true);
    }
}