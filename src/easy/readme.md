# 1. Two sum

## Description
Given an array of integers, return indices of the two numbers such that they add up to a specific target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

## Examples
```text
Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,

return [0, 1].
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let nums = vec![2, 7, 11, 15];
  let target = 9;
  let result = easy::two_sum::two_sum(nums, target);
  println!("result: {:?}", result);
```


# 9. Palindrome number

## Description
Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.

## Examples
```text
Input: 121
Output: true

Input: -121
Output: false

Input: 10
Output: false
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let x = 121;
  let result = easy::palindrome_number::is_palindrome(x);
  println!("result: {}", result);
```


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

# 20. Valid parentheses

## Description
Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

- Open brackets must be closed by the same type of brackets.
- Open brackets must be closed in the correct order.
- Note that an empty string is also considered valid.

## Examples
```text
Input: "()"
Output: true

Input: "()[]{}"
Output: true

Input: "(]"
Output: false
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let s = String::from("()[]{}");
  let result = easy::valid_parentheses::is_valid(s);
  println!("result: {}", result);
```


# 26. Remove duplicates from sorted array

## Description
Given a sorted array nums, remove the duplicates in-place such that each element appear only once and return the new length.

Do not allocate extra space for another array, you must do this by **modifying the input array in-place** with O(1) extra memory.

## Examples
```text
Given nums = [1,1,2],
Output: 2

Given nums = [0,0,1,1,1,2,2,3,3,4],
Output: 5
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
  let result = easy::remove_duplicates_from_sorted_array::remove_duplicates(&mut nums);
  println!("result: {}", result);
```


# 21. Merge two sorted lists

## Description

Merge two sorted linked lists and return it as a new list. The new list should be made by splicing together the nodes of the first two lists.

## Examples
```text
Input: 1->2->4, 1->3->4
Output: 1->1->2->3->4->4
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let mut l1 = Some(Box::new(ListNode::new(1)));
  let mut node2 = Some(Box::new(ListNode::new(2)));
  let node3 = Some(Box::new(ListNode::new(4)));
  node2.as_mut().unwrap().next = node3;
  l1.as_mut().unwrap().next = node2;

  let mut l2 = Some(Box::new(ListNode::new(1)));
  let mut node2 = Some(Box::new(ListNode::new(3)));
  let node3 = Some(Box::new(ListNode::new(4)));
  node2.as_mut().unwrap().next = node3;
  l2.as_mut().unwrap().next = node2;

  let result = easy::merge_two_sorted_lists::merge_two_lists(l1, l2);
  println!("result: {:?}", result);
```

# 27. Remove element

## Description

Given an array nums and a value val, remove all instances of that value in-place and return the new length.

Do not allocate extra space for another array, you must do this by **modifying the input array in-place** with O(1) extra memory.

The order of elements can be changed. It doesn't matter what you leave beyond the new length.

## Examples
```text
Given nums = [3,2,2,3], val = 3,

Your function should return length = 2, with the first two elements of nums being 2.

It doesn't matter what you leave beyond the returned length.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let mut nums = vec![3, 2, 2, 3];
  let val = 3;
  let result = easy::remove_element::remove_element(&mut nums, val);
  println!("result: {:?}", result);
```

# 28. Implement strStr()

## Description

Given a haystack string and a needle string, find the first occurrence of needle in haystack. If no needle is found, return -1.

## Examples
```text
Input: haystack = "hello", needle = "ll"
Output: 2

Input: haystack = "aaaaa", needle = "bba"
Output: -1
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let haystack = String::from("hello");
  let needle = String::from("ll");
  let result = easy::implement_strstr::str_str(haystack, needle);
  println!("result: {:?}", result);
```

# 35. Search insert position

## Description

Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You may assume no duplicates in the array.

## Examples
```text
Input: [1,3,5,6], 5
Output: 2

Input: [1,3,5,6], 2
Output: 1
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let nums = vec![1, 3, 5, 6];
  let target = 5;
  let result = easy::search_insert_position::search_insert(nums, target);
  println!("result: {:?}", result);
```


# 234. Palindrome linked list

## Description
Given a singly linked list, determine if it is a palindrome.

## Examples
```text
Input: 1->2
Output: false

Input: 1->2->2->1
Output: true
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let mut head = Some(Box::new(ListNode::new(1)));
  let mut node2 = Some(Box::new(ListNode::new(2)));
  let mut node3 = Some(Box::new(ListNode::new(2)));
  let node4 = Some(Box::new(ListNode::new(1)));
  node3.as_mut().unwrap().next = node4;
  node2.as_mut().unwrap().next = node3;
  head.as_mut().unwrap().next = node2;
  let result = easy::palindrome_linked_list::is_palindrome(head);
  println!("result: {}", result);
```
