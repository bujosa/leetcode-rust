#![allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn add_next(&mut self, val: i32) {
        let next_node = Some(Box::new(ListNode::new(val)));
        self.next = next_node;
    }
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    dummy.as_mut().unwrap().next = head;
    let mut fast = dummy.clone();
    let mut slow = dummy.as_mut();
    for _ in 0..n {
        fast = fast.unwrap().next;
    }
    while fast.as_ref().unwrap().next.is_some() {
        fast = fast.unwrap().next;
        slow = slow.unwrap().next.as_mut();
    }
    slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.as_mut().unwrap().next.take();
    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth_from_end() {
        let mut l1 = Some(Box::new(ListNode::new(1)));
        let mut l2 = Some(Box::new(ListNode::new(2)));
        let mut l3 = Some(Box::new(ListNode::new(3)));
        let mut l4 = Some(Box::new(ListNode::new(4)));
        let l5 = Some(Box::new(ListNode::new(5)));
        l4.as_mut().unwrap().next = l5;
        l3.as_mut().unwrap().next = l4;
        l2.as_mut().unwrap().next = l3;
        l1.as_mut().unwrap().next = l2;
        let mut r1 = Some(Box::new(ListNode::new(1)));
        let mut r2 = Some(Box::new(ListNode::new(2)));
        let mut r3 = Some(Box::new(ListNode::new(3)));
        let r5 = Some(Box::new(ListNode::new(5)));
        r3.as_mut().unwrap().next = r5;
        r2.as_mut().unwrap().next = r3;
        r1.as_mut().unwrap().next = r2;
        assert_eq!(remove_nth_from_end(l1, 2), r1);
    }
}
