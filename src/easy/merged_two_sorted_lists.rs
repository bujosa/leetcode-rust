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

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list1 = list1;
    let mut list2 = list2;
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut head;
    while list1.is_some() || list2.is_some() {
        let mut val1 = 0;
        let mut val2 = 0;

        if let Some(node) = list1 {
            val1 = node.val;
            list1 = node.next;
        }

        if let Some(node) = list2 {
            val2 = node.val;
            list2 = node.next;
        }

        if val1 < val2 {
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(val1)));
            tail = &mut tail.as_mut().unwrap().next;
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(val2)));
            tail = &mut tail.as_mut().unwrap().next;
        } else {
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(val2)));
            tail = &mut tail.as_mut().unwrap().next;
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(val1)));
            tail = &mut tail.as_mut().unwrap().next;
        }
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
}