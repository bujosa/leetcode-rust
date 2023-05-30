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

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
fn test_add_two_numbers() {
    let l1 = to_list(vec![2, 4, 3]);
    let l2 = to_list(vec![5, 6, 4]);
    let l3 = to_list(vec![7, 0, 8]);
    assert_eq!(add_two_numbers(l1, l2), l3);
}
