#![allow(dead_code)]
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    if gas.iter().sum::<i32>() < cost.iter().sum::<i32>() {
        return -1;
    }

    let mut cur_sum = 0;
    let mut res = 0;
    for i in 0..gas.len() {
        cur_sum += gas[i] - cost[i];
        if cur_sum < 0 {
            cur_sum = 0;
            res = i + 1;
        }
    }

    res as i32
}

#[test]
fn test_can_complete_circuit() {
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];
    assert_eq!(can_complete_circuit(gas, cost), 3);

    let gas = vec![2, 3, 4];
    let cost = vec![3, 4, 3];
    assert_eq!(can_complete_circuit(gas, cost), -1);
}
