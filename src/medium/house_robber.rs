#![allow(dead_code)]
pub fn rob(nums: Vec<i32>) -> i32 {
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
    assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
}
