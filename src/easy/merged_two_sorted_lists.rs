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

// Consider the two list arrays can be different length
pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = list1;
    let mut l2 = list2;
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut head;

    while l1.is_some() && l2.is_some() {
        let mut node: Option<Box<ListNode>>;
        if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
            node = l1;
            l1 = node.as_mut().unwrap().next.take();
        } else {
            node = l2;
            l2 = node.as_mut().unwrap().next.take();
        }
        tail.as_mut().unwrap().next = node;
        tail = &mut tail.as_mut().unwrap().next;
    }

    if l1.is_some() {
        tail.as_mut().unwrap().next = l1;
    } else {
        tail.as_mut().unwrap().next = l2;
    }

    head.unwrap().next

        
}

#[cfg(test)]
fn to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut head;
    for i in v {
        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(i)));
        tail = &mut tail.as_mut().unwrap().next;
    }
    head.unwrap().next
}

#[test]
fn test_merge_two_lists() {
    let l1 = to_list(vec![1, 2, 4]);
    let l2 = to_list(vec![1, 3, 4]);
    let l3 = to_list(vec![1, 1, 2, 3, 4, 4]);
    assert_eq!(merge_two_lists(l1, l2), l3);

    let l1 = to_list(vec![1, 2, 4]);
    let l2 = to_list(vec![1, 3, 4, 5, 6]);
    let l3 = to_list(vec![1, 1, 2, 3, 4, 4, 5, 6]);
    assert_eq!(merge_two_lists(l1, l2), l3);
}