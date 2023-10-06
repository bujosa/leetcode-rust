mod medium;

fn main() {
    use leetcode::hard::merge_k_sorted_lists::{merge_k_lists, Node};
    let lists = vec![
        Node::from_vec(vec![1, 4, 5]),
        Node::from_vec(vec![1, 3, 4]),
        Node::from_vec(vec![2, 6]),
    ];
    let result = merge_k_lists(lists);
    println!("result: {:?}", result);
}
