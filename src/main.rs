mod medium;

use medium::reorder_list::ListNode;

fn main() {
    // Reorder List - Medium
    let mut head = Some(Box::new(
        ListNode::new(1).add_next(
            ListNode::new(2)
                .add_next(ListNode::new(3).add_next(ListNode::new(4).add_next(ListNode::new(5)))),
        ),
    ));

    medium::reorder_list::reorder_list(&mut head);

    // Print elements of the list
    let mut curr = head;
    while let Some(node) = curr {
        println!("{}", node.val);
        curr = node.next;
    }
}
