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


pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut queue = std::collections::VecDeque::new();
    let mut depth = 0;
    
    queue.push_back(root.clone());
    
    while !queue.is_empty() {
        let mut size = queue.len();
        while size > 0 {
            let node = queue.pop_front().unwrap();
            if node.is_some() {
                let node = node.unwrap();
                let node = node.borrow();
                queue.push_back(node.left.clone());
                queue.push_back(node.right.clone());
            }
            size -= 1;
        }
        depth += 1;
    }
    depth - 1
  
}

/*
    Algorithm - BFS
      - Check if root is None, if so return 0
        - Create a queue and push root into it
        - Create a depth variable and set it to 0
        - While queue is not empty
          - Create a size variable and set it to queue length
          - While size is greater than 0
            - Pop the front of the queue and store it in a variable node
            - If node is not None
              - Get the node's left and right child and push them into the queue
            - Decrement size by 1
          - Increment depth by 1
        - Return depth - 1
        
    Complexity
        - Time: O(n)
        - Space: O(n)
 */



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut root = TreeNode::new(3);
        let mut left = TreeNode::new(9);
        let right = TreeNode::new(20);
        let left_left = TreeNode::new(15);
        let left_right = TreeNode::new(7);

        left.left = Some(Rc::new(RefCell::new(left_left)));
        left.right = Some(Rc::new(RefCell::new(left_right)));
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(max_depth(Some(Rc::new(RefCell::new(root)))), 3);
    }

    #[test]
    fn test_2() {
        let mut root = TreeNode::new(1);
        let left = TreeNode::new(2);

        root.left = Some(Rc::new(RefCell::new(left)));

        assert_eq!(max_depth(Some(Rc::new(RefCell::new(root)))), 2);
    }

    #[test]
    fn test_3() {
        let root = TreeNode::new(0);

        assert_eq!(max_depth(Some(Rc::new(RefCell::new(root)))), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(max_depth(None), 0);
    }
}