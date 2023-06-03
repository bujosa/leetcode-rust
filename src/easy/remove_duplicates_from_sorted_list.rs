#![allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut current = head.as_mut();
    while let Some(node) = current {
        while let Some(next) = node.next.as_mut() {
            if node.val == next.val {
                node.next = next.next.take();
            } else {
                break;
            }
        }
        current = node.next.as_mut();
    }
    head
        
}

/*
    Algorithm:
        - Iteration
    Time: O(n)
    Space: O(1)
 */

 #[cfg(test)]
fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vec.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

#[cfg(test)]
fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut head = head;
    let mut vec = vec![];
    while let Some(node) = head {
        vec.push(node.val);
        head = node.next;
    }
    vec
}

#[test]
fn to_list_test() {
    assert_eq!(to_vec(to_list(vec![1, 2, 3])), vec![1, 2, 3]);
    assert_eq!(to_vec(to_list(vec![1, 2, 3, 3, 4, 4, 5])), vec![1, 2, 3, 3, 4, 4, 5]);
}

