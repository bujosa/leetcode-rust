#![allow(dead_code)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut head;
    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let mut sum = carry;

        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }

        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }

        carry = sum / 10;

        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));

        tail = &mut tail.as_mut().unwrap().next;
    }
    head.unwrap().next
}

/*
    Algorithm - Description

    1. Create a new list with a dummy head node
    2. Create a tail pointer to the dummy head
    3. Create a carry variable to hold the carry value
    4. Loop through both lists and add the values together
    5. If the sum is greater than 10, set carry to 1

    Time Complexity - O(n)
    Space Complexity - O(n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(result, ListNode::from_vec(vec![7, 0, 8]));
    }
}
