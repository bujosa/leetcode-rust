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

    #[inline]
    pub fn add_next(mut self, next: Self) -> Self {
        self.next = Some(Box::new(next));
        self
    }
}

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    #[inline(always)]
    fn get_list_middle(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut fast, mut slow) = (&head.clone(), head);
        while fast.is_some() {
            fast = &(fast.as_ref().unwrap().next);
            if fast.is_some() {
                fast = &fast.as_ref().unwrap().next;
                slow = &mut (slow.as_mut().unwrap().next);
            }
        }
        slow.as_mut().unwrap().next.take()
    }

    #[inline(always)]
    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut curr) = head {
            head = curr.next;
            curr.next = prev;
            prev = Some(curr);
        }
        prev
    }

    #[inline(always)]
    fn merge_lists(head1: &mut Option<Box<ListNode>>, head2: Option<Box<ListNode>>) {
        let mut h1 = head1;
        let mut h2 = head2;
        while h1.is_some() && h2.is_some() {
            let h1next = h1.as_mut().unwrap().next.take();
            let h2next = h2.as_mut().unwrap().next.take();
            h1.as_mut().unwrap().next = h2;
            h1.as_mut().unwrap().next.as_mut().unwrap().next = h1next;
            h1 = &mut (h1.as_mut().unwrap().next.as_mut().unwrap().next);
            h2 = h2next;
        }
    }

    let mut head2 = get_list_middle(head);
    head2 = reverse_list(head2);
    merge_lists(head, head2);
}

/*
    Algorithm - Two Pointers

    1. Get the middle of the list
    2. Reverse the second half of the list
    3. Merge the two lists

    Time: O(n)
    Space: O(1)
*/

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
