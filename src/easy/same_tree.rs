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


pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack = Vec::new();
    stack.push((p, q));
    while let Some((p, q)) = stack.pop() {
        if p.is_none() && q.is_none() {
            continue;
        }
        if p.is_none() || q.is_none() {
            return false;
        }
        let p = p.unwrap();
        let q = q.unwrap();
        if p.borrow().val != q.borrow().val {
            return false;
        }
        stack.push((p.borrow().left.clone(), q.borrow().left.clone()));
        stack.push((p.borrow().right.clone(), q.borrow().right.clone()));
    }
    true
}

/*
    Algorithm:
        1. Create a stack
        2. Push p and q to stack
        3. While stack is not empty
            1. Pop p and q from stack
            2. If p and q are None
                1. Continue
            3. If p or q is None
                1. Return false
            4. If p's value is not equal to q's value
                1. Return false
            5. Push p's left and q's left to stack
            6. Push p's right and q's right to stack
        4. Return true

    Time complexity: O(n)
    Space complexity: O(n)
 */

#[test]
fn test_is_same_tree() {
    let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert_eq!(is_same_tree(tree1, tree2), true);

    let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    assert_eq!(is_same_tree(tree1, tree2), false);

    let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    tree1.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    tree2.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    assert_eq!(is_same_tree(tree1, tree2), false);

    let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    tree1.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    tree2.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    assert_eq!(is_same_tree(tree1, tree2), true);

    let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    tree1.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    tree2.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    assert_eq!(is_same_tree(tree1, tree2), false);
}