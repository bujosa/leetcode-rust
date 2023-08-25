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

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {}

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
