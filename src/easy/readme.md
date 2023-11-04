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
  let result = leetcode::easy::two_sum::two_sum(nums, target);
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
  let result = leetcode::easy::palindrome_number::is_palindrome(x);
  println!("result: {}", result);
```

# 13. Roman to integer

## Description

Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

| Symbol | Value |
| :----: | :---: |
|   I    |   1   |
|   V    |   5   |
|   X    |  10   |
|   L    |  50   |
|   C    |  100  |
|   D    |  500  |
|   M    | 1000  |

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
  let result = leetcode::easy::roman_to_integer::roman_to_int(s);
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
  let result = leetcode::easy::longest_common_prefix::longest_common_prefix(strs);
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
  let result = leetcode::easy::valid_parentheses::is_valid(s);
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
  let result = leetcode::easy::remove_duplicates_from_sorted_array::remove_duplicates(&mut nums);
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

  let result = leetcode::easy::merge_two_sorted_lists::merge_two_lists(l1, l2);
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
  let result = leetcode::easy::remove_element::remove_element(&mut nums, val);
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
  let result = leetcode::easy::implement_strstr::str_str(haystack, needle);
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
  let result = leetcode::easy::search_insert_position::search_insert(nums, target);
  println!("result: {:?}", result);
```

# 58. Length of last word

## Description

Given a string s consists of upper/lower-case alphabets and empty space characters ' ', return the length of last word in the string.

If the last word does not exist, return 0.

## Examples

```text
Input: "Hello World"
Output: 5

Input: "a "
Output: 1

Input: "a"
Output: 1

Input: " "
Output: 0
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let s = String::from("Hello World");
  let result = leetcode::easy::length_of_last_word::length_of_last_word(s);
  println!("result: {:?}", result);
```

# 66. Plus one

## Description

Given a non-empty array of digits representing a non-negative integer, plus one to the integer.

The digits are stored such that the most significant digit is at the head of the list, and each element in the array contain a single digit.

You may assume the integer does not contain any leading zero, except the number 0 itself.

## Examples

```text
Input: [1,2,3]
Output: [1,2,4]
Explanation: The array represents the integer 123.

Input: [4,3,2,1]
Output: [4,3,2,2]
Explanation: The array represents the integer 4321.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let digits = vec![1, 2, 3];
  let result = leetcode::easy::plus_one::plus_one(digits);
  println!("result: {:?}", result);
```

# 67. Add binary

## Description

Given two binary strings, return their sum (also a binary string).

The input strings are both **non-empty** and contains only characters 1 or 0.

## Examples

```text
Input: a = "11", b = "1"
Output: "100"

Input: a = "1010", b = "1011"
Output: "10101"
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let a = String::from("11");
  let b = String::from("1");
  let result = leetcode::easy::add_binary::add_binary(a, b);
  println!("result: {:?}", result);
```

# 69. Sqrt(x)

## Description

Implement int sqrt(int x).

Compute and return the square root of x, where x is guaranteed to be a non-negative integer.

Since the return type is an integer, the decimal digits are truncated and only the integer part of the result is returned.

## Examples

```text
Input: 4
Output: 2

Input: 8
Output: 2

Explanation: The square root of 8 is 2.82842..., and since
             the decimal part is truncated, 2 is returned.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let x = 8;
  let result = leetcode::easy::sqrt_x::my_sqrt(x);
  println!("result: {:?}", result);
```

# 70. Climbing stairs

## Description

You are climbing a stair case. It takes n steps to reach to the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

## Examples

```text
Input: 2
Output: 2

Explanation: There are two ways to climb to the top.

1. 1 step + 1 step
2. 2 steps

Input: 3
Output: 3

Explanation: There are three ways to climb to the top.

1. 1 step + 1 step + 1 step
2. 1 step + 2 steps
3. 2 steps + 1 step
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let n = 3;
  let result = leetcode::easy::climbing_stairs::climb_stairs(n);
  println!("result: {:?}", result);
```

# 83. Remove duplicates from sorted list

## Description

Given a sorted linked list, delete all duplicates such that each element appear only once.

## Examples

```text
Input: 1->1->2
Output: 1->2

Input: 1->1->2->3->3
Output: 1->2->3
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let mut head = Some(Box::new(ListNode::new(1)));
  let mut node2 = Some(Box::new(ListNode::new(1)));
  let node3 = Some(Box::new(ListNode::new(2)));
  node2.as_mut().unwrap().next = node3;
  head.as_mut().unwrap().next = node2;
  let result = leetcode::easy::remove_duplicates_from_sorted_list::delete_duplicates(head);
  println!("result: {:?}", result);
```

# 88. Merge sorted array

## Description

You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.

Merge nums1 and nums2 into a single array sorted in non-decreasing order.

The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

## Examples

```text
Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6],       n = 3
Output: [1,2,2,3,5,6]

Input: nums1 = [1], m = 1, nums2 = [], n = 0
Output: [1]
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let mut nums1 = vec![1, 2, 3, 0, 0, 0];
  let m = 3;
  let nums2 = vec![2, 5, 6];
  let n = 3;
  leetcode::easy::merge_sorted_array::merge(&mut nums1, m, nums2, n);
  println!("result: {:?}", nums1);
```

# 94. Binary tree inorder traversal

## Description

Given a binary tree, return the inorder traversal of its nodes' values.

## Examples

```text
Input: [1,null,2,3]
   1
    \
     2
    /
   3

Output: [1,3,2]

Input: [1,2,3,4,5,6,7]
       1
      / \
     2   3
    / \ / \
   4  5 6  7

Output: [4,2,5,1,6,3,7]

```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let mut root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
  let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
  let node3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
  node2.as_mut().unwrap().borrow_mut().left = node3;
  root.as_mut().unwrap().borrow_mut().right = node2;
  let result = leetcode::easy::binary_tree_inorder_traversal::inorder_traversal(root);
  println!("result: {:?}", result);
```

# 100. Same tree

## Description

Given two binary trees, write a function to check if they are the same or not.

Two binary trees are considered the same if they are structurally identical and the nodes have the same value.

## Examples

```text
Input:     1         1
          / \       / \
         2   3     2   3

        [1,2,3],   [1,2,3]

Output: true

Input:     1         1
          /           \
         2             2

        [1,2],     [1,null,2]

Output: false
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let mut root1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
  let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
  let node3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
  node2.as_mut().unwrap().borrow_mut().left = node3;
  root1.as_mut().unwrap().borrow_mut().right = node2;

  let mut root2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
  let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
  let node3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
  node2.as_mut().unwrap().borrow_mut().left = node3;
  root2.as_mut().unwrap().borrow_mut().right = node2;

  let result = leetcode::easy::same_tree::is_same_tree(root1, root2);
  println!("result: {:?}", result);
```

# 101. Symmetric tree

## Description

Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).

## Examples

```text
Input: [1,2,2,3,4,4,3]
    1
   / \
  2   2
 / \ / \
3  4 4  3

Output: true

Input: [1,2,2,null,3,null,3]
    1
   / \
  2   2
   \   \
   3    3

Output: false
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let mut root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
  let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
  let mut node3 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
  let node4 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
  let node5 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
  let node6 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
  let node7 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
  node3.as_mut().unwrap().borrow_mut().left = node6;
  node3.as_mut().unwrap().borrow_mut().right = node7;
  node2.as_mut().unwrap().borrow_mut().left = node4;
  node2.as_mut().unwrap().borrow_mut().right = node5;
  root.as_mut().unwrap().borrow_mut().left = node2;
  root.as_mut().unwrap().borrow_mut().right = node3;
  let result = leetcode::easy::symmetric_tree::is_symmetric(root);
  println!("result: {:?}", result);
```

# 104. Maximum depth of binary tree

## Description

Given a binary tree, find its maximum depth.

The maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.

## Examples

```text
Given binary tree [3,9,20,null,null,15,7],

    3
   / \
  9  20
    /  \
   15   7
Output: 3
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  let mut root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
  let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
  let mut node3 = Some(Rc::new(RefCell::new(TreeNode::new(20))));
  let node4 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
  let node5 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
  node3.as_mut().unwrap().borrow_mut().left = node4;
  node3.as_mut().unwrap().borrow_mut().right = node5;
  root.as_mut().unwrap().borrow_mut().left = node2;
  root.as_mut().unwrap().borrow_mut().right = node3;
  let result = leetcode::easy::maximum_depth_of_binary_tree::max_depth(root);
  println!("result: {:?}", result);
```

# 108. Convert sorted array to binary search tree

## Description

Given an array where elements are sorted in ascending order, convert it to a height balanced BST.

For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.

## Examples

```text
Given the sorted array: [-10,-3,0,5,9],

One possible answer is: [0,-3,9,-10,null,5], which represents the following height balanced BST:

      0
     / \
   -3   9
   /   /
 -10  5
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let nums = vec![-10, -3, 0, 5, 9];
  let result = leetcode::easy::convert_sorted_array_to_binary_search_tree::sorted_array_to_bst(nums);
  println!("result: {:?}", result);
```

# 110. Balanced binary tree

## Description

Given a binary tree, determine if it is height-balanced.

For this problem, a height-balanced binary tree is defined as:

a binary tree in which the depth of the two subtrees of every node never differ by more than 1.

## Examples

```text
Given the following tree [3,9,20,null,null,15,7]:

    3
   / \
  9  20
    /  \
   15   7

Return true.

Given the following tree [1,2,2,3,3,null,null,4,4]:

       1
      / \
     2   2
    / \
   3   3
  / \
 4   4

Return false.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let mut root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
  let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
  let mut node3 = Some(Rc::new(RefCell::new(TreeNode::new(20))));
  let node4 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
  let node5 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
  node3.as_mut().unwrap().borrow_mut().left = node4;
  node3.as_mut().unwrap().borrow_mut().right = node5;
  root.as_mut().unwrap().borrow_mut().left = node2;
  root.as_mut().unwrap().borrow_mut().right = node3;
  let result = leetcode::easy::balanced_binary_tree::is_balanced(root);
  println!("result: {:?}", result);
```

# 111. Minimum depth of binary tree

## Description

Given a binary tree, find its minimum depth.

The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.

## Examples

```text
Given binary tree [3,9,20,null,null,15,7],

    3
   / \
  9  20
    /  \
   15   7

return its minimum depth = 2.

Given binary tree [2,null,3,null,4,null,5,null,6],

    2
     \
      3
       \
        4
         \
          5
           \
            6
output: 5
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  let mut root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
  let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
  let mut node3 = Some(Rc::new(RefCell::new(TreeNode::new(20))));
  let node4 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
  let node5 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
  node3.as_mut().unwrap().borrow_mut().left = node4;
  node3.as_mut().unwrap().borrow_mut().right = node5;
  root.as_mut().unwrap().borrow_mut().left = node2;
  root.as_mut().unwrap().borrow_mut().right = node3;
  let result = leetcode::easy::minimum_depth_of_binary_tree::min_depth(root);
  println!("result: {:?}", result);
```

# 112. Path sum

## Description

Given a binary tree and a sum, determine if the tree has a root-to-leaf path such that adding up all the values along the path equals the given sum.

## Examples

```text
Given the below binary tree and sum = 22,

      5
     / \
    4   8
   /   / \
  11  13  4
 /  \      \
7    2      1

return true, as there exist a root-to-leaf path 5->4->11->2 which sum is 22.
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  let mut root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
  let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
  let mut node3 = Some(Rc::new(RefCell::new(TreeNode::new(8))));
  let mut node4 = Some(Rc::new(RefCell::new(TreeNode::new(11))));
  let node5 = Some(Rc::new(RefCell::new(TreeNode::new(13))));
  let mut node6 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
  let node7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
  let node8 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
  let node9 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
  node6.as_mut().unwrap().borrow_mut().right = node9;
  node4.as_mut().unwrap().borrow_mut().left = node7;
  node4.as_mut().unwrap().borrow_mut().right = node8;
  node3.as_mut().unwrap().borrow_mut().left = node5;
  node3.as_mut().unwrap().borrow_mut().right = node6;
  node2.as_mut().unwrap().borrow_mut().left = node4;
  root.as_mut().unwrap().borrow_mut().left = node2;
  root.as_mut().unwrap().borrow_mut().right = node3;
  let result = leetcode::easy::path_sum::has_path_sum(root, 22);
  println!("result: {:?}", result);
```

# 118. Pascal's triangle

## Description

Given a non-negative integer numRows, generate the first numRows of Pascal's triangle.

[![Pascal's triangle](../../theory/images/pascals_triangle.gif)](https://en.wikipedia.org/wiki/Pascal%27s_triangle)

## Examples

```text
Input: 5
Output:
[
     [1],
    [1,1],
   [1,2,1],
  [1,3,3,1],
 [1,4,6,4,1]
]
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  let result = leetcode::easy::pascals_triangle::generate(5);
  println!("result: {:?}", result);
```

# 119. Pascal's triangle II

## Description

Given a non-negative index k where k ≤ 33, return the kth index row of the Pascal's triangle.

Note that the row index starts from 0.

[![Pascal's triangle](../../theory/images/pascals_triangle.gif)](https://en.wikipedia.org/wiki/Pascal%27s_triangle)

## Examples

```text
Input: 3
Output: [1,3,3,1]
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  let result = leetcode::easy::pascals_triangle::get_row(3);
  println!("result: {:?}", result);
```

# 121. Best time to buy and sell stock

## Description

Say you have an array for which the ith element is the price of a given stock on day i.

If you were only permitted to complete at most one transaction (i.e., buy one and sell one share of the stock), design an algorithm to find the maximum profit.

Note that you cannot sell a stock before you buy one.

## Examples

```text
Input: [7,1,5,3,6,4]
Output: 5

Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
             Not 7-1 = 6, as selling price needs to be larger than buying price.
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  let result = leetcode::easy::best_time_to_buy_and_sell_stock::max_profit(vec![7, 1, 5, 3, 6, 4]);
  println!("result: {:?}", result);
```

# 125. Valid palindrome

## Description

Given a string, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.

Note: For the purpose of this problem, we define empty string as valid palindrome.

## Examples

```text
Input: s = "A man, a plan, a canal: Panama"
Output: true
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
let mut leetcode::easy::valid_palindrome::is_palindrome("race a car".to_string());
  println!("result: {:?}", result);
```

# 141. Linked list cycle

## Description

Given a linked list, determine if it has a cycle in it.

To represent a cycle in the given linked list, we use an integer pos which represents the position (0-indexed) in the linked list where tail connects to. If pos is -1, then there is no cycle in the linked list.

## Examples

```text
Input: head = [3,2,0,-4], pos = 1
Output: true

Explanation: There is a cycle in the linked list, where tail connects to the second node.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let mut head = Some(Box::new(ListNode::new(3)));
  let mut node2 = Some(Box::new(ListNode::new(2)));
  let mut node3 = Some(Box::new(ListNode::new(0)));
  let node4 = Some(Box::new(ListNode::new(-4)));
  node3.as_mut().unwrap().next = node4;
  node2.as_mut().unwrap().next = node3;
  head.as_mut().unwrap().next = node2;
  let result = leetcode::easy::linked_list_cycle::has_cycle(head);
  println!("result: {:?}", result);
```

# 155. Min stack

## Description

Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

- push(x) -- Push element x onto stack.
- pop() -- Removes the element on top of the stack.
- top() -- Get the top element.
- getMin() -- Retrieve the minimum element in the stack.

## Examples

```text
Input:
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]

Output:
[null,null,null,null,-3,null,0,-2]

Explanation:

MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin(); // return -3
minStack.pop();
minStack.top();    // return 0
minStack.getMin(); // return -2
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  let mut min_stack = leetcode::easy::min_stack::MinStack::new();
  min_stack.push(-2);
  min_stack.push(0);
  min_stack.push(-3);
  let result = min_stack.get_min();
  println!("result: {:?}", result);
  min_stack.pop();
  let result = min_stack.top();
  println!("result: {:?}", result);
  let result = min_stack.get_min();
  println!("result: {:?}", result);
```

# 206. Reverse linked list

## Description

Reverse a singly linked list.

## Examples

```text
Input: 1->2->3->4->5->NULL
Output: 5->4->3->2->1->NULL
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let mut head = Some(Box::new(ListNode::new(1)));
  let mut node2 = Some(Box::new(ListNode::new(2)));
  let mut node3 = Some(Box::new(ListNode::new(3)));
  let mut node4 = Some(Box::new(ListNode::new(4)));
  let node5 = Some(Box::new(ListNode::new(5)));
  node4.as_mut().unwrap().next = node5;
  node3.as_mut().unwrap().next = node4;
  node2.as_mut().unwrap().next = node3;
  head.as_mut().unwrap().next = node2;
  let result = leetcode::easy::reverse_linked_list::reverse_list(head);
  println!("result: {:?}", result);
```

# 217. Contains duplicate

## Description

Given an array of integers, find if the array contains any duplicates.

Your function should return true if any value appears at least twice in the array, and it should return false if every element is distinct.

## Examples

```text
Input: [1,2,3,1]
Output: true

Input: [1,2,3,4]
Output: false

Input: [1,1,1,3,3,4,3,2,4,2]
Output: true
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  let result = leetcode::easy::contains_duplicate::contains_duplicate(vec![1, 2, 3, 1]);
  println!("result: {:?}", result);
```

# 226. Invert binary tree

## Description

Invert a binary tree.

## Examples

```text
Input:

     4
   /   \
  2     7
 / \   / \
1   3 6   9

Output:

     4
   /   \
  7     2
 / \   / \
9   6 3   1
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  use leetcode::easy::invert_binary_tree::{TreeNode, invert_tree};
  let mut root = Some(Rc::new(RefCell::new(TreeNode::new(4))));
  let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
  let mut node3 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
  let node4 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
  let node5 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
  let node6 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
  let node7 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
  node3.as_mut().unwrap().borrow_mut().left = node6;
  node3.as_mut().unwrap().borrow_mut().right = node7;
  node2.as_mut().unwrap().borrow_mut().left = node4;
  node2.as_mut().unwrap().borrow_mut().right = node5;
  root.as_mut().unwrap().borrow_mut().left = node2;
  root.as_mut().unwrap().borrow_mut().right = node3;
  let result = invert_tree(root);
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
  let result = leetcode::easy::palindrome_linked_list::is_palindrome(head);
  println!("result: {}", result);
```

# 242. Valid anagram

## Description

Given two strings s and t , write a function to determine if t is an anagram of s.

## Examples

```text
Input: s = "anagram", t = "nagaram"

Output: true

Input: s = "rat", t = "car"

Output: false
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  let result = leetcode::easy::valid_anagram::is_anagram(String::from("anagram"), String::from("nagaram"));
  println!("result: {:?}", result);
```

# 543. Diameter of binary tree

## Description

Given a binary tree, you need to compute the length of the diameter of the tree. The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.

## Examples

```text
Given a binary tree

          1
         / \
        2   3
       / \
      4   5

Return 3, which is the length of the path [4,2,1,3] or [5,2,1,3].
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  use leetcode::easy::diameter_of_binary_tree::{TreeNode, diameter_of_binary_tree};
  let mut root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
  let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
  let node3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
  let node4 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
  let node5 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
  node2.as_mut().unwrap().borrow_mut().left = node4;
  node2.as_mut().unwrap().borrow_mut().right = node5;
  root.as_mut().unwrap().borrow_mut().left = node2;
  root.as_mut().unwrap().borrow_mut().right = node3;
  let result = diameter_of_binary_tree(root);
  println!("result: {:?}", result);
```

# 572. Subtree of another tree

## Description

Given two non-empty binary trees s and t, check whether tree t has exactly the same structure and node values with a subtree of s. A subtree of s is a tree consists of a node in s and all of this node's descendants. The tree s could also be considered as a subtree of itself.

## Examples

```text
Given tree s:

     3
    / \
   4   5
  / \
 1   2

Given tree t:

   4
  / \
 1   2

Return true, because t has the same structure and node values with a subtree of s.
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  use std::{cell::RefCell, rc::Rc};
  use leetcode::easy::subtree_of_another_tree::{TreeNode, is_subtree};
  let mut root1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
  let mut node2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
  let node3 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
  let node4 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
  let node5 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
  node2.as_mut().unwrap().borrow_mut().left = node4;
  node2.as_mut().unwrap().borrow_mut().right = node5;
  root1.as_mut().unwrap().borrow_mut().left = node2;
  root1.as_mut().unwrap().borrow_mut().right = node3;

  let mut root2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
  let node6 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
  let node7 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
  root2.as_mut().unwrap().borrow_mut().left = node6;
  root2.as_mut().unwrap().borrow_mut().right = node7;

  let result = is_subtree(root1, root2);
  println!("result: {:?}", result);
```

# 703. Kth largest element in a stream

## Description

Design a class to find the `kth` largest element in a stream. Note that it is the kth largest element in the sorted order, not the kth distinct element.

Implement KthLargest class:
- `KthLargest(int k, int[] nums)` Initializes the object with the integer k and the stream of integers `nums`.
- `int add(int val)` Appends the integer `val` to the stream and returns the element representing the `kth` largest element in the stream.

## Examples

```text
Input
["KthLargest", "add", "add", "add", "add", "add"]
[[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
Output
[null, 4, 5, 5, 8, 8]

Explanation
KthLargest kthLargest = new KthLargest(3, [4, 5, 8, 2]);
kthLargest.add(3);   // return 4
kthLargest.add(5);   // return 5
kthLargest.add(10);  // return 5
kthLargest.add(9);   // return 8
kthLargest.add(4);   // return 8
```

# 704. Binary search

## Description

Given a **sorted** (in ascending order) integer array nums of n elements and a target value, write a function to search target in nums. If target exists, then return its index, otherwise return -1.

## Examples

```text
Input: nums = [-1,0,3,5,9,12], target = 9
Output: 4

Explanation: 9 exists in nums and its index is 4

Input: nums = [-1,0,3,5,9,12], target = 2
Output: -1

Explanation: 2 does not exist in nums so return -1
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  let result = leetcode::easy::binary_search::search(vec![-1, 0, 3, 5, 9, 12], 9);
  println!("result: {:?}", result);
```

# 746. Min cost climbing stairs

## Description

On a staircase, the `i`-th step has some non-negative cost `cost[i]` assigned (0 indexed).

Once you pay the cost, you can either climb one or two steps. You need to find minimum cost to reach the top of the floor, and you can either start from the step with index 0, or the step with index 1.

## Examples

```text
Input: cost = [10, 15, 20]
Output: 15

Explanation: Cheapest is start on cost[1], pay that cost and go to the top.
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  let result = leetcode::easy::min_cost_climbing_stairs::min_cost_climbing_stairs(vec![10, 15, 20]);
  println!("result: {:?}", result);
```


# 1046. Last stone weight

## Description

We have a collection of stones, each stone has a positive integer weight.

Each turn, we choose the two **heaviest** stones and smash them together. Suppose the stones have weights `x` and `y` with `x <= y`. The result of this smash is:

- If `x == y`, both stones are totally destroyed;
- If `x != y`, the stone of weight `x` is totally destroyed, and the stone of weight `y` has new weight `y-x`.

At the end, there is at most 1 stone left. Return the weight of this stone (or 0 if there are no stones left.)

## Examples

Example 1:
```text
Input: stones = [2,7,4,1,8,1]
Output: 1
Explanation: 
We combine 7 and 8 to get 1 so the array converts to [2,4,1,1,1] then,
we combine 2 and 4 to get 2 so the array converts to [2,1,1,1] then,
we combine 2 and 1 to get 1 so the array converts to [1,1,1] then,
we combine 1 and 1 to get 0 so the array converts to [1] then that's the value of the last stone.
```

Example 2:
```text
Input: stones = [1]
Output: 1
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
  let result = leetcode::easy::last_stone_weight::last_stone_weight(vec![2, 7, 4, 1, 8, 1]);
  println!("result: {:?}", result);
```