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
    let mut lists = lists;
    let mut res = None;
    let mut res_tail = &mut res;
    loop {
        let mut min = std::i32::MAX;
        let mut min_idx = 0;
        let mut all_none = true;
        for (i, list) in lists.iter().enumerate() {
            if let Some(node) = list {
                all_none = false;
                if node.val < min {
                    min = node.val;
                    min_idx = i;
                }
            }
        }
        if all_none {
            break;
        }
        let mut node = lists[min_idx].take().unwrap();
        lists[min_idx] = node.next.take();
        *res_tail = Some(node);
        res_tail = &mut res_tail.as_mut().unwrap().next;
    }
    res
}

/*
    Algorithm - O(nlogk)

    1. Create a min heap of size k
    2. Insert the first element of each list into the heap
    3. Pop the min element from the heap and insert the next element from the list of the popped element
    4. Repeat step 3 until all the lists are empty

    Time Complexity - O(nlogk)
    Space Complexity - O(k)
*/

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
