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

/// Reverses a singly linked list.
///
/// # Arguments
///
/// * `head` - A singly linked list represented as an `Option<Box<ListNode>>`.
///
/// # Example
///
/// ```
/// use leetcode::easy::reverse_linked_list::{reverse_list, ListNode};
///
/// let mut node1 = ListNode::new(1);
/// let mut node2 = ListNode::new(2);
/// let mut node3 = ListNode::new(3);
/// let mut node4 = ListNode::new(4);
/// let mut node5 = ListNode::new(5);
///
/// node4.next = Some(Box::new(node5));
/// node3.next = Some(Box::new(node4));
/// node2.next = Some(Box::new(node3));
/// node1.next = Some(Box::new(node2));
///
/// let reversed_list = reverse_list(Some(Box::new(node1)));
///
/// assert_eq!(reversed_list.as_ref().unwrap().val, 5);
/// assert_eq!(reversed_list.as_ref().unwrap().next.as_ref().unwrap().val, 4);
/// ```
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;
    while let Some(mut curr_node) = curr {
        let next_temp = curr_node.next.take();
        curr_node.next = prev;
        prev = Some(curr_node);
        curr = next_temp;
    }
    prev
}

/*
    Algorithm - Iterative
    - Create a prev variable and set it to None
    - Create a curr variable and set it to head
    - While curr is not None
        - Create a next_temp variable and set it to curr.next
        - Set curr.next to prev
        - Set prev to curr
        - Set curr to next_temp
    - Return prev

    Analysis
        - Time Complexity: O(n)
        - Space Complexity: O(1)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_206() {
        assert_eq!(reverse_list(None), None);
        assert_eq!(
            reverse_list(Some(Box::new(ListNode { val: 1, next: None }))),
            Some(Box::new(ListNode { val: 1, next: None }))
        );
        assert_eq!(
            reverse_list(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            }))),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None }))
            }))
        );
        assert_eq!(
            reverse_list(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None }))
                }))
            }))
        );
    }
}
