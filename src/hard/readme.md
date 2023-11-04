# 4. Median of Two Sorted Arrays

## Description

Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.

The overall run time complexity should be O(log (m+n)).

## Examples

Example 1:

```rust
Input: nums1 = [1,3], nums2 = [2]
Output: 2.00000
Explanation: merged array = [1,2,3] and median is 2.
```

Example 2:

```rust
Input: nums1 = [1,2], nums2 = [3,4]
Output: 2.50000
Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
```

Example 3:

```rust
Input: nums1 = [0,0], nums2 = [0,0]
Output: 0.00000
```

Example 4:

```rust
Input: nums1 = [], nums2 = [1]
Output: 1.00000
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let result = leetcode::hard::median_of_two_sorted_arrays::find_median_sorted_arrays(nums1, nums2);
    println!("result: {:?}", result);
```

# 23. Merge k Sorted Lists

## Description

You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.

Merge all the linked-lists into one sorted linked-list and return it.

## Examples

Example 1:

```rust
Input: lists = [[1,4,5],[1,3,4],[2,6]]
Output: [1,1,2,3,4,4,5,6]
Explanation: The linked-lists are:
[
  1->4->5,
  1->3->4,
  2->6
]

merging them into one sorted list:
1->1->2->3->4->4->5->6
```

Example 2:

```rust
Input: lists = []
Output: []
```

Example 3:

```rust
Input: lists = [[]]
Output: []
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    use leetcode::hard::merge_k_sorted_lists::{Node, merge_k_lists};
    let lists = vec![
        Node::from_vec(vec![1, 4, 5]),
        Node::from_vec(vec![1, 3, 4]),
        Node::from_vec(vec![2, 6]),
    ];
    let result = merge_k_lists(lists);
    println!("result: {:?}", result);
```

# 25. Reverse Nodes in k-Group

## Description

Given a linked list, reverse the nodes of a linked list k at a time and return its modified list.

k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.

You may not alter the values in the list's nodes, only nodes themselves may be changed.

## Examples

Example 1:

```rust
Input: head = [1,2,3,4,5], k = 2
Output: [2,1,4,3,5]
```

Example 2:

```rust
Input: head = [1,2,3,4,5], k = 3
Output: [3,2,1,4,5]
```

Example 3:

```rust
Input: head = [1,2,3,4,5], k = 1
Output: [1,2,3,4,5]
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    use leetcode::hard::reverse_nodes_in_k_group::{Node, reverse_k_group};
    let head = Node::from_vec(vec![1, 2, 3, 4, 5]);
    let k = 2;
    let result = reverse_k_group(head, k);
    println!("result: {:?}", result);
```

# 42. Trapping Rain Water

## Description

Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.

## Examples

Example 1:

```
Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6
Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
```

Example 2:

```
Input: height = [4,2,0,3,2,5]
Output: 9
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let result = leetcode::hard::trapping_rain_water::trap(height);
    println!("result: {:?}", result);
```


# 51. N-Queens

## Description

The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.

Given an integer n, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.

Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.

## Examples

Example 1:

![img](../../theory/images/51-n-queens.jpg)

```rust
Input: n = 4
Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above
```

Example 2:

```rust
Input: n = 1
Output: [["Q"]]s
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    let n = 4;
    let result = leetcode::hard::n_queens::solve_n_queens(n);
    println!("result: {:?}", result);
```


# 76. Minimum Window Substring

## Description

Given two strings s and t of lengths m and n respectively, return the minimum window substring of s such that every character in t (including duplicates) is included in the window. If there is no such substring, return the empty string "".

The testcases will be generated such that the answer is unique.

A substring is a contiguous sequence of characters within the string.

## Examples

Example 1:

```
Input: s = "ADOBECODEBANC", t = "ABC"
Output: "BANC"
Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.
```

Example 2:

```
Input: s = "a", t = "a"
Output: "a"
Explanation: The entire string s is the minimum window.
```

Example 3:

```
Input: s = "a", t = "aa"
Output: ""
Explanation: Both 'a's from t must be included in the window.
Since the largest window of s only has one 'a', return empty string.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    let s = String::from("ADOBECODEBANC");
    let t = String::from("ABC");
    let result = leetcode::hard::minimum_window_substring::min_window(s, t);
    println!("result: {:?}", result);
```

# 84. Largest Rectangle in Histogram

## Description

Given an array of integers heights representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram.

## Examples

Example 1:

![img](https://assets.leetcode.com/uploads/2021/01/04/histogram.jpg)

```
Input: heights = [2,1,5,6,2,3]
Output: 10
Explanation: The above is a histogram where width of each bar is 1.
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

````rust
    let heights = vec![2, 1, 5, 6, 2, 3];
    let result = leetcode::hard::largest_rectangle_in_histogram::largest_rectangle_area(heights);
    println!("result: {:?}", result);
    ```
````

# 212. Word Search II

## Description

Given an m x n board of characters and a list of strings words, return all words on the board.

Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.

## Examples

Example 1:

![img](../../theory/images/word_search_ii.jpg)

```rust
Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]
Output: ["eat","oath"]
```

# 239. Sliding Window Maximum

## Description

You are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position.

Return the max sliding window.

## Examples

Example 1:

```
Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
Output: [3,3,5,5,6,7]
Explanation:
Window position                Max
---------------               -----
[1  3  -1] -3  5  3  6  7       3
 1 [3  -1  -3] 5  3  6  7       3
 1  3 [-1  -3  5] 3  6  7       5
 1  3  -1 [-3  5  3] 6  7       5
 1  3  -1  -3 [5  3  6] 7       6
 1  3  -1  -3  5 [3  6  7]      7
```

Example 2:

```
Input: nums = [1], k = 1
Output: [1]
```

Example 3:

```
Input: nums = [1,-1], k = 1
Output: [1,-1]
```

Example 4:

```
Input: nums = [9,11], k = 2
Output: [11]
```

Example 5:

```
Input: nums = [4,-2], k = 2
Output: [4]
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let result = leetcode::hard::sliding_window_maximum::max_sliding_window(nums, k);
    println!("result: {:?}", result);
```

# 295. Find Median from Data Stream

## Description

The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value and the median is the mean of the two middle values.

- For example, for arr = [2,3,4], the median is 3.
- For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.

Implement the MedianFinder class:

- MedianFinder() initializes the MedianFinder object.
- void addNum(int num) adds the integer num from the data stream to the data structure.
- double findMedian() returns the median of all elements so far. Answers within 10-5 of the actual answer will be accepted.

## Examples

Example 1:

```rust
Input
["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
[[], [1], [2], [], [3], []]
Output
[null, null, null, 1.5, null, 2.0]

Explanation
MedianFinder medianFinder = new MedianFinder();
medianFinder.addNum(1);    // arr = [1]
medianFinder.addNum(2);    // arr = [1, 2]
medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
medianFinder.addNum(3);    // arr[1, 2, 3]
medianFinder.findMedian(); // return 2.0
```

## How to Run in main.rs

Put the code below in main.rs and run `cargo run`

```rust
    let mut median_finder = leetcode::hard::find_median_from_data_stream::MedianFinder::new();
    median_finder.add_num(1);
    median_finder.add_num(2);
    let result = median_finder.find_median();
    println!("result: {:?}", result);
```