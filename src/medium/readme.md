# 2. Add two numbers

## Description

You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit.

Add the two numbers and return it as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

## Examples

```text
Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
Output: 7 -> 0 -> 8

Explanation: 342 + 465 = 807.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let l1 = ListNode::new(2)
    .add_next(ListNode::new(4))
    .add_next(ListNode::new(3));
  let l2 = ListNode::new(5)
    .add_next(ListNode::new(6))
    .add_next(ListNode::new(4));
  let result = medium::add_two_numbers::add_two_numbers(l1, l2);
  println!("result: {:?}", result);
```

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

# 22. Generate Parentheses

## Description

Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

## Examples

```text
Input: n = 3
Output: [
  "((()))",
  "(()())",
  "(())()",
  "()(())",
  "()()()"
]

Input: n = 1
Output: ["()"]
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let n = 3;
  let result = medium::generate_parentheses::generate_parenthesis(n);
  println!("result: {:?}", result);
```

# 36. Valid Sudoku

## Description

Determine if a 9x9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

1. Each row must contain the digits 1-9 without repetition.
2. Each column must contain the digits 1-9 without repetition.
3. Each of the 9 3x3 sub-boxes of the grid must contain the digits 1-9 without repetition.

## Examples

```text
Input:
[
  ["5","3",".",".","7",".",".",".","."],
  ["6",".",".","1","9","5",".",".","."],
  [".","9","8",".",".",".",".","6","."],
  ["8",".",".",".","6",".",".",".","3"],
  ["4",".",".","8",".","3",".",".","1"],
  ["7",".",".",".","2",".",".",".","6"],
  [".","6",".",".",".",".","2","8","."],
  [".",".",".","4","1","9",".",".","5"],
  [".",".",".",".","8",".",".","7","9"]
]
Output: true
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let board = vec![
    vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
    vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
    vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
    vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
    vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
    vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
    vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
    vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
    vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
  ];
  let result = medium::is_valid_sudoku::is_valid_sudoku(board);
  println!("result: {}", result);
```

# 49. Group Anagrams

## Description

Given an array of strings, group anagrams together.

## Examples

```text
Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
Output:
[
  ["ate","eat","tea"],
  ["nat","tan"],
  ["bat"]
]
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
  let result = medium::group_anagrams::group_anagrams(strs);
  println!("result: {:?}", result);
```

# 128. Longest Consecutive Sequence

## Description

Given an unsorted array of integers, find the length of the longest consecutive elements sequence.

Your algorithm should run in O(n) complexity.

## Examples

```text
Input: [100, 4, 200, 1, 3, 2]
Output: 4

Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let nums = vec![100, 4, 200, 1, 3, 2];
  let result = hard::longest_consecutive::longest_consecutive(nums);
  println!("result: {}", result);
```

# 150. Evaluate Reverse Polish Notation

## Description

Evaluate the value of an arithmetic expression in Reverse Polish Notation.

Note that:

- Valid operators are +, -, \*, /. Each operand may be an integer or another expression.
- Division between two integers should truncate toward zero.
- The given RPN expression is always valid. That means the expression would always evaluate to a result and there won't be any divide by zero operation.
- The input represents a valid arithmetic expression containing only integers and operators (+, -, \*, /).
- Each operand may be an integer or another expression.

## Examples

```text
Input: ["2", "1", "+", "3", "*"]
Output: 9
Explanation: ((2 + 1) * 3) = 9
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let tokens = vec!["2", "1", "+", "3", "*"];
  let result = medium::eval_rpn::eval_rpn(tokens);
  println!("result: {}", result);
```

# 167. Two Sum II - Input array is sorted

## Description

Given an array of integers that is already sorted in ascending order, find two numbers such that they add up to a specific target number.

The function twoSum should return indices of the two numbers such that they add up to the target, where index1 must be less than index2.

## Examples

```text
Input: numbers = [2,7,11,15], target = 9
Output: [1,2]

Explanation: The sum of 2 and 7 is 9. Therefore index1 = 1, index2 = 2.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let numbers = vec![2, 7, 11, 15];
  let target = 9;
  let result = easy::two_sum::two_sum(numbers, target);
  println!("result: {:?}", result);
```

# 238. Product of Array Except Self

## Description

Given an array nums of n integers where n > 1, return an array output such that output[i] is equal to the product of all the elements of nums except nums[i].

## Examples

```text
Input:  [1,2,3,4]
Output: [24,12,8,6]
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let nums = vec![1, 2, 3, 4];
  let result = medium::product_except_self::product_except_self(nums);
  println!("result: {:?}", result);
```

# 271. Encode and Decode Strings

## Description

Design an algorithm to encode a list of strings to a string. The encoded string is then sent over the network and is decoded back to the original list of strings.

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let strs = vec!["Hello", "World"];
  let result = medium::encode_and_decode_strings::encode(strs.clone());
  println!("result: {}", result);
  let result = medium::encode_and_decode_strings::decode(result);
  println!("result: {:?}", result);
```

# 347. Top K Frequent Elements

## Description

Given a non-empty array of integers, return the k most frequent elements.

## Examples

```text
Input: nums = [1,1,1,2,2,3], k = 2
Output: [1,2]
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let nums = vec![1, 1, 1, 2, 2, 3];
  let k = 2;
  let result = medium::top_k_frequent::top_k_frequent(nums, k);
  println!("result: {:?}", result);
```

# 739. Daily Temperatures

## Description

Given a list of daily temperatures T, return a list such that, for each day in the input, tells you how many days you would have to wait until a warmer temperature. If there is no future day for which this is possible, put 0 instead.

## Examples

```text
Input: T = [73, 74, 75, 71, 69, 72, 76, 73]
Output: [1, 1, 4, 2, 1, 1, 0, 0]
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let t = vec![73, 74, 75, 71, 69, 72, 76, 73];
  let result = medium::daily_temperatures::daily_temperatures(t);
  println!("result: {:?}", result);
```

# 853. Car Fleet

## Description

N cars are going to the same destination along a one lane road. The destination is target miles away.

Each car i has a constant speed speed[i] (in miles per hour), and initial position position[i] miles towards the target along the road.

A car can never pass another car ahead of it, but it can catch up to it, and drive bumper to bumper at the same speed.

The distance between these two cars is ignored - they are assumed to have the same position.

A car fleet is some non-empty set of cars driving at the same position and same speed. Note that a single car is also a car fleet.

If a car catches up to a car fleet right at the destination point, it will still be considered as one car fleet.

How many car fleets will arrive at the destination?

## Examples

```text
Input: target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]
Output: 3

Explanation:
The cars starting at 10 and 8 become a fleet, meeting each other at 12.
The car starting at 0 doesn't catch up to any other car, so it is a fleet by itself.
The cars starting at 5 and 3 become a fleet, meeting each other at 6.
Note that no other cars meet these fleets before the destination, so the answer is 3.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let target = 12;
  let position = vec![10, 8, 0, 5, 3];
  let speed = vec![2, 4, 1, 1, 3];
  let result = medium::car_fleet::car_fleet(target, position, speed);
  println!("result: {}", result);
```
