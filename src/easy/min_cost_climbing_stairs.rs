#![allow(dead_code)]
pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
    cost.push(0);
    for i in (0..cost.len() - 2).rev() {
        cost[i] += std::cmp::min(cost[i + 1], cost[i + 2]);
    }
    std::cmp::min(cost[0], cost[1])
}

#[test]
fn test_min_cost_climbing_stairs() {
    assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    assert_eq!(
        min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}
