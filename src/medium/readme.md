# 15. Three sum

## Description

Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0?

Find all unique triplets in the array which gives the sum of zero.

Note:
The solution set must not contain duplicate triplets.

## Examples
```text
Given array nums = [-1, 0, 1, 2, -1, -4],
Output:
[
  [-1, 0, 1],
  [-1, -1, 2]
]
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let nums = vec![-1, 0, 1, 2, -1, -4];
  let result = easy::three_sum::three_sum(nums);
  println!("result: {:?}", result);
```

# 16. Three sum closest

## Description

Given an array nums of n integers and an integer target, find three integers in nums such that the sum is closest to target.

Return the sum of the three integers.

You may assume that each input would have exactly one solution.

## Examples
```text
Given array nums = [-1, 2, 1, -4], and target = 1.
The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let nums = vec![-1, 2, 1, -4];
  let target = 1;
  let result = medium::three_sum_closest::three_sum_closest(nums, target);
  println!("result: {}", result);
```