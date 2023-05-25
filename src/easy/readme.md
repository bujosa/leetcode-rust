# 13. Roman to integer

## Description
Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

| Symbol | Value |
| :----: | :---: |
| I      | 1     |
| V      | 5     |  
| X      | 10    |
| L      | 50    |
| C      | 100   |
| D      | 500   |
| M      | 1000  |

For example, two is written as II in Roman numeral, just two one's added together.

Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII,

which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for

four is not IIII. Instead, the number four is written as IV. Because the one is before the five we

subtract it making four. The same principle applies to the number nine, which is written as IX.

There are six instances where subtraction is used:

- I can be placed before V (5) and X (10) to make 4 and 9.
- X can be placed before L (50) and C (100) to make 40 and 90.
- C can be placed before D (500) and M (1000) to make 400 and 900.

Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range from 1 to 3999.

## Examples
```text
Input: "III"
Output: 3

Input: "IV"
Output: 4

Input: "IX"
Output: 9
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let s = String::from("III");
  let result = easy::roman_to_integer::roman_to_int(s);
  println!("result: {}", result);
```


# 14. Longest common prefix

## Description
Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".

## Examples
```text
Input: ["flower","flow","flight"]
Output: "fl"

Input: ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
  let result = easy::longest_common_prefix::longest_common_prefix(strs);
  println!("result: {}", result);
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