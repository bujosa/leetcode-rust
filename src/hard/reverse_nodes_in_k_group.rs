#![allow(dead_code)]
// Definition for singly-linked list.
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

    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}

pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut sentinel = ListNode::new(-1);
    let mut last_solution_node = &mut sentinel;

    'finish: loop {
        for _ in 0..k {
            if let Some(mut current) = head.take() {
                head = current.next.take();
                current.next = last_solution_node.next.take();
                last_solution_node.next = Some(current);
            } else {
                break 'finish;
            }
        }
        while let Some(ref mut next) = last_solution_node.next {
            last_solution_node = next;
        }
    }

    let mut final_reversal_head = ListNode::new(-1);
    while let Some(mut current) = last_solution_node.next.take() {
        last_solution_node.next = current.next.take();
        current.next = final_reversal_head.next.take();
        final_reversal_head.next = Some(current);
    }
    last_solution_node.next = final_reversal_head.next.take();
    sentinel.next.take()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_25() {
        assert_eq!(
            reverse_k_group(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 2),
            ListNode::from_vec(vec![2, 1, 4, 3, 5])
        );
        assert_eq!(
            reverse_k_group(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 3),
            ListNode::from_vec(vec![3, 2, 1, 4, 5])
        );
        assert_eq!(
            reverse_k_group(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 1),
            ListNode::from_vec(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(
            reverse_k_group(ListNode::from_vec(vec![1]), 1),
            ListNode::from_vec(vec![1])
        );
    }
}
