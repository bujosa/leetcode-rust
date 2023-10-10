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

/*
    Algorithm - O(n) time O(1) space

    1. Create a sentinel node
    2. Create a last_solution_node that points to the sentinel node
    3. Loop through the list
        1. Create a counter that counts up to k
        2. If the counter is less than k
            1. If the current node is not None
                1. Set the current node to the next node
                2. Set the current node's next to the last_solution_node's next
                3. Set the last_solution_node's next to the current node
            2. Else
                1. Break out of the loop
        3. While the last_solution_node's next is not None
            1. Set the last_solution_node to the next node
    4. Create a final_reversal_head node
    5. While the last_solution_node's next is not None
        1. Set the current node to the last_solution_node's next
        2. Set the last_solution_node's next to the current node's next
        3. Set the current node's next to the final_reversal_head's next
        4. Set the final_reversal_head's next to the current node

    6. Set the last_solution_node's next to the final_reversal_head's next
    7. Return the sentinel node's next

    Time - O(n)
    Space - O(1)
*/

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
