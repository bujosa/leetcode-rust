#![allow(dead_code)]

pub fn trap(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_capacity = 0;
    let mut left_max = height[left];
    let mut right_max = height[right];

    while left < right {
        if left_max < right_max {
            left += 1;
            left_max = left_max.max(height[left]);
            max_capacity += left_max - height[left];
        } else {
            right -= 1;
            right_max = right_max.max(height[right]);
            max_capacity += right_max - height[right];
        }
    }

    return max_capacity;
}

/*
    Algorithm - Two Pointers

    Time    O(N)
    Space   O(1)

    Explanation:
    - The idea is to use two pointers to track the left and right side of the container
    - The capacity of the container is determined by the shorter side

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap(height), 6);
    }

    #[test]
    fn test_trap_2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(trap(height), 9);
    }
}
