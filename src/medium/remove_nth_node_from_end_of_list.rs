#![allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn add_next(&mut self, val: i32) {
        let next_node = Some(Box::new(ListNode::new(val)));
        self.next = next_node;
    }

    pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        for i in v {
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(i)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        head.unwrap().next
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

/*
   Algorithm - Two Pointers

   1. Create a dummy node and set its next to head
   2. Create two pointers, fast and slow, and set them to dummy
   3. Move fast n times
   4. Move fast and slow until fast.next is None
   5. Remove slow.next
   6. Return dummy.next

   Time: O(n)
   Space: O(1)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth_from_end() {
        let l1 = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let r1 = ListNode::from_vec(vec![1, 2, 3, 5]);
        assert_eq!(remove_nth_from_end(l1, 2), r1);
    }
}
