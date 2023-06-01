#![allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut v = Vec::new();
    let mut current = head;
    while let Some(node) = current {
        v.push(node.val);
        current = node.next;
    }
    let mut i = 0;
    let mut j = v.len() - 1;
    while i < j {
        if v[i] != v[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

#[cfg(test)]
fn create_list_node(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in v.iter().rev() {
        let mut node = ListNode {
            val: *i,
            next: None
        };
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

#[test]
fn test_is_palindrome() {
    let v = vec![1, 2];
    let head = create_list_node(v);
    let result = is_palindrome(head);
    assert_eq!(result, false);

    let v = vec![1, 2, 2, 1];
    let head = create_list_node(v);
    let result = is_palindrome(head);
    assert_eq!(result, true);
}