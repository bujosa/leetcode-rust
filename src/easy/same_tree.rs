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
}

pub fn is_same_tree(p: Tree, q: Tree) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            p.borrow().val == q.borrow().val
                && is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                && is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
        }
        _ => false,
    }
}

/*
   Algorithm - Recursion
    - If both trees are empty then they are same
    - If both trees are non-empty
        - Check if current data of both trees are same
        - Recursively check if left subtree of both trees are same
        - Recursively check if right subtree of both trees are same
    - If one of them is empty and other is not, then they are not same

    Complexity
        - Time is O(n) where n is the number of nodes in the tree
        - Space is O(n) where n is the number of nodes in the tree
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
