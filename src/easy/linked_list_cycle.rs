#![allow(dead_code)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    let mut slow = &head;
    let mut fast = &head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;

        if slow == fast {
            return true; // Cycle detected
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_141() {
        assert_eq!(has_cycle(None), false);
    }

    #[test]
    fn test_141_2() {
        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(2);
        let mut node3 = ListNode::new(3);
        let mut node4 = ListNode::new(4);
        let node5 = ListNode::new(5);

        node4.next = Some(Box::new(node5));
        node3.next = Some(Box::new(node4));
        node2.next = Some(Box::new(node3));
        node1.next = Some(Box::new(node2));

        assert_eq!(has_cycle(Some(Box::new(node1))), false);
    }

    #[test]
    fn test_141_3() {
        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(2);
        let mut node3 = ListNode::new(3);
        let mut node4 = ListNode::new(4);
        let mut node5 = ListNode::new(5);

        node1.next = Some(Box::new(node2.clone())); // Clone node2
        node2.next = Some(Box::new(node3.clone())); // Clone node3
        node3.next = Some(Box::new(node4.clone())); // Clone node4
        node4.next = Some(Box::new(node5.clone())); // Clone node5
        node5.next = Some(Box::new(node1.clone())); // Clone node1

        assert_eq!(has_cycle(Some(Box::new(node1))), false);
    }
}
