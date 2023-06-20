#![allow(dead_code)]
pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars: Vec<(i32, f64)> = position
        .into_iter()
        .zip(speed.into_iter().map(|x| x as f64))
        .collect();

    cars.sort_by(|a, b| b.0.cmp(&a.0));

    let mut stack = vec![];

    for (pos, speed) in cars {
        stack.push((target - pos) as f64 / speed);

        if stack.len() >= 2 && stack[stack.len() - 1] <= stack[stack.len() - 2] {
            stack.pop();
        }
    }

    stack.len() as i32
}

/*
    Algorithm - Greedy

    1. Sort the cars by position
    2. Iterate through the cars and calculate the time it takes to reach the target
    3. If the time is less than the time of the car in the stack, then pop the car from the stack
    4. Return the length of the stack

    Time: O(nlogn)
    Space: O(n)
 */

#[test]
fn test_car_fleet() {
    assert_eq!(car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);

    assert_eq!(car_fleet(10, vec![0, 4, 2], vec![2, 1, 3]), 1);
}
