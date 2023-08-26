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

/// Merges two sorted linked lists into a single sorted linked list.
///
/// # Arguments
///
/// * `list1` - A linked list of type `Option<Box<ListNode>>`.
/// * `list2` - A linked list of type `Option<Box<ListNode>>`.
///
/// # Example
///
/// ```
/// use leetcode::easy::merge_two_sorted_lists::{merge_two_lists, ListNode};
///
/// let l1 = Some(Box::new(ListNode {
///     val: 1,
///     next: Some(Box::new(ListNode {
///         val: 2,
///         next: Some(Box::new(ListNode { val: 4, next: None })),
///     })),
/// }));
///
/// let l2 = Some(Box::new(ListNode {
///     val: 1,
///     next: Some(Box::new(ListNode {
///         val: 3,
///         next: Some(Box::new(ListNode { val: 4, next: None })),
///     })),
/// }));
///
/// let expected = Some(Box::new(ListNode {
///     val: 1,
///     next: Some(Box::new(ListNode {
///         val: 1,
///         next: Some(Box::new(ListNode {
///             val: 2,
///             next: Some(Box::new(ListNode {
///                 val: 3,
///                 next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode { val: 4, next: None })) })),
///             })),
///         })),
///     })),
/// }));
///
/// assert_eq!(merge_two_lists(l1, l2), expected);
/// ```
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
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

/*
    Algorithm - Iterative
    - Create a head variable and set it to a new ListNode with value 0
    - Create a tail variable and set it to head
    - While l1 and l2 are not None
        - Create a node variable
        - If l1.val < l2.val
            - Set node to l1
            - Set l1 to node.next
        - Else
            - Set node to l2
            - Set l2 to node.next
        - Set tail.next to node
        - Set tail to tail.next

    - If l1 is not None
        - Set tail.next to l1
    - Else
        - Set tail.next to l2

    - Return head.next

    Analysis
        - Time Complexity: O(n + m)
        - Space Complexity: O(1)
*/

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
