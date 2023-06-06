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


pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() == 0 {
        return None;
    }
    let mid = nums.len() / 2;
    let mut root = TreeNode::new(nums[mid]);
    root.left = sorted_array_to_bst(nums[..mid].to_vec());
    root.right = sorted_array_to_bst(nums[mid+1..].to_vec());
    Some(Rc::new(RefCell::new(root)))
}

/*
    Algorithm:
        - Find the middle element of the array
        - Create a node with the middle element as the value
        - Recursively call the function on the left and right subarrays
        - Return the node
    
    Complexity:
        - Time: O(n)
        - Space: O(n)
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_108() {
        assert_eq!(sorted_array_to_bst(vec![-10,-3,0,5,9]), Some(Rc::new(RefCell::new(TreeNode{
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: -10,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        }))));
    }
}



