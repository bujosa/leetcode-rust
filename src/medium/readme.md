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
