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
    use leetcode::medium::add_two_numbers::ListNode;
    let l1 = ListNode::from_vec(vec![2, 4, 3]);
    let l2 = ListNode::from_vec(vec![5, 6, 4]);
    let result = leetcode::medium::add_two_numbers::add_two_numbers(l1, l2);
    println!("result: {:?}", result);
```

# 3. Longest Substring Without Repeating Characters

## Description

Given a string, find the length of the longest substring without repeating characters.

## Examples

```text
Input: "abcabcbb"
Output: 3

Explanation: The answer is "abc", with the length of 3.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let s = String::from("abcabcbb");
  let result = leetcode::medium::longest_substring_without_repeating_characters::length_of_longest_substring(s);
  println!("result: {}", result);
```

# 11. Container With Most Water

## Description

Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai). n vertical lines are drawn such that the two endpoints of line i is at (i, ai) and (i, 0). Find two lines, which together with x-axis forms a container, such that the container contains the most water.

Note: You may not slant the container and n is at least 2.

## Examples

```text
Input: [1,8,6,2,5,4,8,3,7]
Output: 49
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
  let result = leetcode::medium::container_with_most_water::max_area(height);
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
  let result = leetcode::medium::three_sum::three_sum(nums);
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
  let result = leetcode::medium::three_sum_closest::three_sum_closest(nums, target);
  println!("result: {}", result);
```

# 19. Remove Nth Node From End of List

## Description

Given a linked list, remove the n-th node from the end of list and return its head.

## Examples

```text
Given linked list: 1->2->3->4->5, and n = 2.
After removing the second node from the end, the linked list becomes 1->2->3->5.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    use leetcode::medium::remove_nth_node_from_end_of_list::ListNode;
    let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let result = leetcode::medium::remove_nth_node_from_end_of_list::remove_nth_from_end(head, 2);
    println!("result: {:?}", result);
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
  let result = leetcode::medium::generate_parentheses::generate_parenthesis(n);
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
  let result = leetcode::medium::valid_sudoku::is_valid_sudoku(board);
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
    let strs = strs.iter().map(|s| s.to_string()).collect();
    let result = leetcode::medium::group_anagrams::group_anagrams(strs);
    println!("result: {:?}", result);
```

# 74. Search a 2D Matrix

## Description

Write an efficient algorithm that searches for a value in an m x n matrix. This matrix has the following properties:

- Integers in each row are sorted from left to right.
- The first integer of each row is greater than the last integer of the previous row.

## Examples

```text
Input:
matrix = [
  [1,   3,  5,  7],
  [10, 11, 16, 20],
  [23, 30, 34, 50]
]
target = 3
Output: true
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let matrix = vec![
    vec![1, 3, 5, 7],
    vec![10, 11, 16, 20],
    vec![23, 30, 34, 50],
  ];
  let target = 3;
  let result = leetcode::medium::search_a_2d_matrix::search_matrix(matrix, target);
  println!("result: {}", result);
```

# 98. Validate Binary Search Tree

## Description

Given a binary tree, determine if it is a valid binary search tree (BST).

Assume a BST is defined as follows:

- The left subtree of a node contains only nodes with keys less than the node's key.
- The right subtree of a node contains only nodes with keys greater than the node's key.
- Both the left and right subtrees must also be binary search trees.

## Examples

```text
Input:
    2
   / \
  1   3
Output: true

Input:
    5
   / \
  1   4
     / \
    3   6
Output: false

Explanation: The input is: [5,1,4,null,null,3,6]. The root node's value
             is 5 but its right child's value is 4.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    use std::{cell::RefCell, rc::Rc};

    use leetcode::medium::validate_binary_search_tree::{is_valid_bst, TreeNode};

    let mut root = TreeNode::new(2);
    let mut node1 = TreeNode::new(1);
    let mut node3 = TreeNode::new(3);

    node1.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node1.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left = Some(Rc::new(RefCell::new(node1)));
    root.right = Some(Rc::new(RefCell::new(node3)));

    let result = is_valid_bst(root);
    println!("result: {}", result);
```


# 102. Binary Tree Level Order Traversal

## Description

Given a binary tree, return the level order traversal of its nodes' values. (ie, from left to right, level by level).

## Examples

```text
Input: [3,9,20,null,null,15,7]
Output:
[
  [3],
  [9,20],
  [15,7]
]
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    use leetcode::medium::binary_tree_level_order_traversal::{level_order, TreeNode};
    let root = TreeNode::from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
    ]);

    let result = level_order(root);
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
  let result = leetcode::medium::longest_consecutive_sequence::longest_consecutive(nums);
  println!("result: {}", result);
```

# 138. Copy List with Random Pointer

## Description

A linked list of length n is given such that each node contains an additional random pointer, which could point to any node in the list, or null.

Construct a deep copy of the list. The deep copy should consist of exactly n brand new nodes, where each new node has its value set to the value of its corresponding original node. Both the next and random pointer of the new nodes should point to new nodes in the copied list such that the pointers in the original list and copied list represent the same list state. None of the pointers in the new list should point to nodes in the original list.

For example, if there are two nodes X and Y in the original list, where `X.random --> Y`, then for the corresponding two nodes x and y in the copied list,`x.random --> y`.

Return the head of the copied linked list.

The linked list is represented in the input/output as a list of n nodes. Each node is represented as a pair of `[val, random_index]` where:

`val`: an integer representing Node.val
`random_index`: the index of the node (range from 0 to n-1) that the random pointer points to, or null if it does not point to any node.
Your code will only be given the head of the original linked list.

## Examples

```text
Input:
{"$id":"1","next":{"$id":"2","next":null,"random":{"$ref":"2"},"val":2},"random":{"$ref":"2"},"val":1}

Explanation:
Node 1's value is 1, both of its next and random pointer points to Node 2.
Node 2's value is 2, its next pointer points to null and its random pointer points to itself.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    use std::{cell::RefCell, rc::Rc};

    use leetcode::medium::copy_list_with_random_pointer::{copy_random_list, Node};

    let mut node1 = Some(Rc::new(RefCell::new(Node::new(1))));
    let mut node2 = Some(Rc::new(RefCell::new(Node::new(2))));
    let mut node3 = Some(Rc::new(RefCell::new(Node::new(3))));

    node1.as_ref().unwrap().next = node2.clone();
    node2.as_ref().unwrap().next = node3.clone();
    node1.as_ref().unwrap().random = Some(node3.clone());
    node2.as_ref().unwrap().random = Some(node1.clone());
    node3.as_ref().unwrap().random = Some(node2.clone());

    copy_random_list(node1);
```

# 143. Reorder List

## Description

Given a singly linked list L: L0→L1→…→Ln-1→Ln, reorder it to: L0→Ln→L1→Ln-1→L2→Ln-2→…

You may not modify the values in the list's nodes, only nodes itself may be changed.

## Examples

```text
Given 1->2->3->4, reorder it to 1->4->2->3.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    use leetcode::medium::reorder_list::ListNode;
    let mut head = Some(Box::new(
        ListNode::new(1).add_next(
            ListNode::new(2)
                .add_next(ListNode::new(3).add_next(ListNode::new(4).add_next(ListNode::new(5)))),
        ),
    ));
    leetcode::medium::reorder_list::reorder_list(&mut head);

    let mut curr = head;
    while let Some(node) = curr {
        println!("{}", node.val);
        curr = node.next;
    }
```

# 146. LRU Cache

## Description

Design and implement a data structure for Least Recently Used (LRU) cache. It should support the following operations: get and put.

get(key) - Get the value (will always be positive) of the key if the key exists in the cache, otherwise return -1.

put(key, value) - Set or insert the value if the key is not already present. When the cache reached its capacity, it should invalidate the least recently used item before inserting a new item.

The cache is initialized with a positive capacity.

Follow up:
Could you do both operations in O(1) time complexity?

## Examples

```text
LRUCache cache = new LRUCache( 2 /* capacity */ );

cache.put(1, 1);
cache.put(2, 2);
cache.get(1);       // returns 1
cache.put(3, 3);    // evicts key 2
cache.get(2);       // returns -1 (not found)
cache.put(4, 4);    // evicts key 1
cache.get(1);       // returns -1 (not found)
cache.get(3);       // returns 3
cache.get(4);       // returns 4
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let mut cache = leetcode::medium::lru_cache::LRUCache::new(2);
  cache.put(1, 1);
  cache.put(2, 2);
  println!("{}", cache.get(1)); // returns 1
  cache.put(3, 3); // evicts key 2
  println!("{}", cache.get(2)); // returns -1 (not found)
  cache.put(4, 4); // evicts key 1
  println!("{}", cache.get(1)); // returns -1 (not found)
  println!("{}", cache.get(3)); // returns 3
  println!("{}", cache.get(4)); // returns 4
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
    let tokens = vec!["2", "1", "+", "3", "*"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let result = leetcode::medium::evaluate_reverse_polish_notation::eval_rpn(tokens);
    println!("result: {}", result);
```

# 153. Find Minimum in Rotated Sorted Array

## Description

Suppose an array of length n sorted in ascending order is rotated between 1 and n times. For example, the array nums = [0,1,2,4,5,6,7] might become:

- `[4,5,6,7,0,1,2]` if it was rotated 4 times.
- `[0,1,2,4,5,6,7]` if it was rotated 7 times.

Notice that rotating an array `[a[0], a[1], a[2], ..., a[n-1]]` 1 time results in the array `[a[n-1], a[0], a[1], a[2], ..., a[n-2]]`.

Given the sorted rotated array nums of unique elements, return the minimum element of this array.

You must write an algorithm that runs in O(log n) time.

## Examples

```text
Input: [3,4,5,1,2]
Output: 1

Input: [4,5,6,7,0,1,2]
Output: 0
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let nums = vec![3, 4, 5, 1, 2];
  let result = leetcode::medium::find_min::find_min(nums);
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
  let result = leetcode::medium::two_sum_ii_input_array_is_sorted::two_sum(numbers, target);
  println!("result: {:?}", result);
```

# 199. Binary Tree Right Side View

## Description

Given a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.

## Examples

```text
Input: [1,2,3,null,5,null,4]
Output: [1, 3, 4]

Explanation:

   1            <---
 /   \
2     3         <---
  \     \
    5     4       <---
  ```
## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    use leetcode::medium::binary_tree_right_side_view::{right_side_view, TreeNode};
    let root = TreeNode::from_vec(vec![
        Some(1),
        Some(2),
        Some(3),
        None,
        Some(5),
        None,
        Some(4),
    ]);
    let result = right_side_view(root);
    println!("result: {:?}", result);
```


# 235. Lowest Common Ancestor of a Binary Search Tree

## Description

Given a binary search tree (BST), find the lowest common ancestor (LCA) of two given nodes in the BST.

According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow **a node to be a descendant of itself**).”

## Examples

```text
Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
Output: 6
Explanation: The LCA of nodes 2 and 8 is 6.

Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
Output: 2
Explanation: The LCA of nodes 2 and 4 is 2, since a node can be a descendant of itself according to the LCA definition.
```

## How to Run in main.rs

Put this code below in main.rs and run `cargo run`

```rust
    use leetcode::medium::lowest_common_ancestor_of_a_binary_search_tree::{lowest_common_ancestor, TreeNode};
    let mut root = TreeNode::new(6);
    let mut node2 = TreeNode::new(2);
    let mut node8 = TreeNode::new(8);
    let mut node0 = TreeNode::new(0);
    let mut node4 = TreeNode::new(4);
    let mut node7 = TreeNode::new(7);
    let mut node9 = TreeNode::new(9);
    let mut node3 = TreeNode::new(3);
    let mut node5 = TreeNode::new(5);

    node2.left = Some(Rc::new(RefCell::new(node0)));
    node2.right = Some(Rc::new(RefCell::new(node4)));
    node4.left = Some(Rc::new(RefCell::new(node3)));
    node4.right = Some(Rc::new(RefCell::new(node5)));
    node8.left = Some(Rc::new(RefCell::new(node7)));
    node8.right = Some(Rc::new(RefCell::new(node9)));
    root.left = Some(Rc::new(RefCell::new(node2)));
    root.right = Some(Rc::new(RefCell::new(node8)));

    let p = Some(Rc::new(RefCell::new(node2)));
    let q = Some(Rc::new(RefCell::new(node8)));
    let result = lowest_common_ancestor(root, p, q);
    println!("result: {}", result.unwrap().borrow().val);
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
  let result = leetcode::medium::product_of_array_except_self::product_except_self(nums);
  println!("result: {:?}", result);
```

# 271. Encode and Decode Strings

## Description

Design an algorithm to encode a list of strings to a string. The encoded string is then sent over the network and is decoded back to the original list of strings.

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
     let strs = vec![
        "Hello".to_string(),
        "World".to_string(),
        "How".to_string(),
        "Are".to_string(),
        "You".to_string(),
    ];
    let result = leetcode::medium::encode_and_decode_strings::encode(strs);
    println!("result: {}", result);
    let result = leetcode::medium::encode_and_decode_strings::decode(result);
    println!("result: {:?}", result);
```

# 287. Find the Duplicate Number

## Description

Given an array nums containing n + 1 integers where each integer is between 1 and n (inclusive), prove that at least one duplicate number must exist. Assume that there is only one duplicate number, find the duplicate one.

## Examples

```text
Input: [1,3,4,2,2]
Output: 2
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let nums = vec![1, 3, 4, 2, 2];
  let result = leetcode::medium::find_duplicate::find_duplicate(nums);
  println!("result: {}", result);
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
  let result = leetcode::medium::top_k_frequent_elements::top_k_frequent(nums, k);
  println!("result: {:?}", result);
```

# 424. Longest Repeating Character Replacement

## Description

Given a string s that consists of only uppercase English letters, you can perform at most k operations on that string.

In one operation, you can choose any character of the string and change it to any other uppercase English character.

Find the length of the longest sub-string containing all repeating letters you can get after performing the above operations.

## Examples

```text
Input: s = "ABAB", k = 2
Output: 4
Explanation: Replace the two 'A's with two 'B's or vice versa.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let s = String::from("ABAB");
  let k = 2;
  let result = leetcode::medium::longest_repeating_character_replacement::character_replacement(s, k);
  println!("result: {}", result);
```

# 567. Permutation in String

## Description

Given two strings s1 and s2, write a function to return true if s2 contains the permutation of s1. In other words, one of the first string's permutations is the substring of the second string.

## Examples

```text
Input: s1 = "ab" s2 = "eidbaooo"
Output: True

Explanation: s2 contains one permutation of s1 ("ba").
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let s1 = String::from("ab");
  let s2 = String::from("eidbaooo");
  let result = leetcode::medium::permutation_in_string::check_inclusion(s1, s2);
  println!("result: {}", result);
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
  let result = leetcode::medium::daily_temperatures::daily_temperatures(t);
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
  let result = leetcode::medium::car_fleet::car_fleet(target, position, speed);
  println!("result: {}", result);
```

# 875. Koko Eating Bananas

## Description

Koko loves to eat bananas. There are n piles of bananas, the ith pile has piles[i] bananas. The guards have gone and will come back in h hours.

Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.

Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.

Return the minimum integer k such that she can eat all the bananas within h hours.

## Examples

```text
Input: piles = [3,6,7,11], h = 8
Output: 4

Input: piles = [30,11,23,4,20], h = 5
Output: 30

Input: piles = [30,11,23,4,20], h = 6
Output: 23
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
  let piles = vec![3, 6, 7, 11];
  let h = 8;
  let result = leetcode::medium::koko_eating_bananas::min_eating_speed(piles, h);
  println!("result: {}", result);
```

# 981. Time Based Key-Value Store

## Description

Design a time-based key-value data structure that can store multiple values for the same key at different time stamps and retrieve the key's value at a certain timestamp.

Implement the TimeMap class:

- `TimeMap()` Initializes the object of the data structure.
- `void set(String key, String value, int timestamp)` Stores the key `key` with the value `value` at the given time `timestamp`.
- `String get(String key, int timestamp)` Returns a value such that `set` was called previously, with `timestamp_prev <= timestamp`. If there are multiple such values, it returns the value associated with the largest `timestamp_prev`. If there are no values, it returns `""`.

## Examples

```text
Input
["TimeMap", "set", "get", "get", "set", "get", "get"]
[[], ["foo", "bar", 1], ["foo", 1], ["foo", 3], ["foo", "bar2", 4], ["foo", 4], ["foo", 5]]
Output
[null, null, "bar", "bar", null, "bar2", "bar2"]

Explanation
TimeMap timeMap = new TimeMap();

timeMap.set("foo", "bar", 1);  // store the key "foo" and value "bar" along with timestamp = 1.
timeMap.get("foo", 1);         // return "bar"
timeMap.get("foo", 3);         // return "bar", since there is no value corresponding to foo at timestamp 3 and timestamp 2, then the only value is at timestamp 1 ie "bar".

timeMap.set("foo", "bar2", 4);
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    let mut time_map = leetcode::medium::time_based_key_value_store::TimeMap::new();
    time_map.set(String::from("foo"), String::from("bar"), 1);
    let result = time_map.get(String::from("foo"), 1);
    println!("result: {}", result);
    let result = time_map.get(String::from("foo"), 3);
    println!("result: {}", result);
    time_map.set(String::from("foo"), String::from("bar2"), 4);
    let result = time_map.get(String::from("foo"), 4);
    println!("result: {}", result);
    let result = time_map.get(String::from("foo"), 5);
    println!("result: {}", result);
```

# 1448. Count Good Nodes in Binary Tree

## Description

Given a binary tree root, a node X in the tree is named good if in the path from root to X there are no nodes with a value greater than X.

Return the number of good nodes in the binary tree.

## Examples

```text
Input: root = [3,1,4,3,null,1,5]
Output: 4

Explanation: Nodes in blue are good.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    use leetcode::medium::count_good_nodes_in_binary_tree::{good_nodes, TreeNode};
    let root = TreeNode::from_vec(vec![
        Some(3),
        Some(1),
        Some(4),
        Some(3),
        None,
        Some(1),
        Some(5),
    ]);
    let result = good_nodes(root);
    println!("result: {}", result);
```

