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

    fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_k_lists_test() {
        let lists = vec![
            ListNode::from_vec(vec![1, 4, 5]),
            ListNode::from_vec(vec![1, 3, 4]),
            ListNode::from_vec(vec![2, 6]),
        ];
        let res = merge_k_lists(lists);
        let expected = ListNode::from_vec(vec![1, 1, 2, 3, 4, 4, 5, 6]);
        assert_eq!(res, expected);
    }
}
