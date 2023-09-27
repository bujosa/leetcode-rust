#![allow(dead_code)]

pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut tortoise = nums[0];
    let mut hare = nums[0];

    loop {
        tortoise = nums[tortoise as usize];
        hare = nums[nums[hare as usize] as usize];

        if tortoise == hare {
            break;
        }
    }

    let mut ptr1 = nums[0];
    let mut ptr2 = tortoise;

    while ptr1 != ptr2 {
        ptr1 = nums[ptr1 as usize];
        ptr2 = nums[ptr2 as usize];
    }

    ptr1
}

/*
    Algorithm - Floyd's Tortoise and Hare (Cycle Detection)
    ------------------------------------------------------
    1. Initialize tortoise and hare to the first element of the array.
    2. Move tortoise to nums[tortoise].
    3. Move hare to nums[nums[hare]].
    4. Repeat steps 2 and 3 until tortoise == hare.
    5. Initialize ptr1 to the first element of the array.
    6. Initialize ptr2 to tortoise.
    7. Move ptr1 to nums[ptr1].
    8. Move ptr2 to nums[ptr2].
    9. Repeat steps 7 and 8 until ptr1 == ptr2.
    10. Return ptr1.

    Time Complexity: O(n)
    Space Complexity: O(1)

    How works Floyd's Tortoise and Hare?
    ------------------------------------
    https://youtu.be/wjYnzkAhcNk?t=145


*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate() {
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }

    #[test]
    fn test_find_duplicate_with_10_values() {
        assert_eq!(find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), 9);
        assert_eq!(find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 8]), 8);
    }
}
