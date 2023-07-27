#![allow(dead_code)]

pub fn trap(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_capacity = 0;
    let mut left_max = 0;
    let mut right_max = 0;

    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                max_capacity += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                max_capacity += right_max - height[right];
            }
            right -= 1;
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
}
