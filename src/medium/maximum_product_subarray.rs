#![allow(dead_code)]
pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut res = *nums.iter().max().unwrap();
    let (mut cur_min, mut cur_max) = (1, 1);

    for &n in nums.iter() {
        let tmp = cur_max * n;
        cur_max = std::cmp::max(std::cmp::max(n * cur_max, n * cur_min), n);
        cur_min = std::cmp::min(std::cmp::min(tmp, n * cur_min), n);
        res = std::cmp::max(std::cmp::max(res, cur_max), cur_min);
    }

    res
}

#[test]
fn test_max_product() {
    assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(max_product(vec![-2, 0, -1]), 0);
    assert_eq!(max_product(vec![-2]), -2);
}
