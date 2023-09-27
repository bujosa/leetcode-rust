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
