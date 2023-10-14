mod medium;

fn main() {
    use leetcode::medium::remove_nth_node_from_end_of_list::ListNode;
    let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let result = leetcode::medium::remove_nth_node_from_end_of_list::remove_nth_from_end(head, 2);
    println!("result: {:?}", result);
}
