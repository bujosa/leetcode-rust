#![allow(dead_code)]
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    if nums.is_empty() {
        return 0;
    }

    std::cmp::max(
        helper_robber_i(nums[0..nums.len() - 1].to_vec()),
        helper_robber_i(nums[1..].to_vec()),
    )
}

fn helper_robber_i(nums: Vec<i32>) -> i32 {
    let (mut rob1, mut rob2) = (0, 0);

    for &n in nums.iter() {
        let temp = std::cmp::max(n + rob1, rob2);
        rob1 = rob2;
        rob2 = temp;
    }

    rob2
}

#[test]
fn test_rob() {
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(rob(vec![2, 7, 9, 3, 1]), 11);
}
