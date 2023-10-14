mod medium;

fn main() {
    use leetcode::medium::add_two_numbers::ListNode;
    let l1 = ListNode::from_vec(vec![2, 4, 3]);
    let l2 = ListNode::from_vec(vec![5, 6, 4]);
    let result = leetcode::medium::add_two_numbers::add_two_numbers(l1, l2);
    println!("result: {:?}", result);
}
