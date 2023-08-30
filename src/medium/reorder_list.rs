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
}

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reorder_list() {
        let mut head = Some(Box::new(ListNode::new(1)));
        let mut node2 = Some(Box::new(ListNode::new(2)));
        let mut node3 = Some(Box::new(ListNode::new(3)));
        let mut node4 = Some(Box::new(ListNode::new(4)));
        let node5 = Some(Box::new(ListNode::new(5)));
        node4.as_mut().unwrap().next = node5;
        node3.as_mut().unwrap().next = node4;
        node2.as_mut().unwrap().next = node3;
        head.as_mut().unwrap().next = node2;
        reorder_list(&mut head);
        let mut node = head;
        assert_eq!(node.as_ref().unwrap().val, 1);
        node = node.as_mut().unwrap().next.clone();
        assert_eq!(node.as_ref().unwrap().val, 5);
        node = node.as_mut().unwrap().next.clone();
        assert_eq!(node.as_ref().unwrap().val, 2);
        node = node.as_mut().unwrap().next.clone();
        assert_eq!(node.as_ref().unwrap().val, 4);
        node = node.as_mut().unwrap().next.clone();
        assert_eq!(node.as_ref().unwrap().val, 3);
        node = node.as_mut().unwrap().next.clone();
        assert_eq!(node, None);
    }
}
