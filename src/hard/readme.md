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
    let result = hard::largest_rectangle_in_histogram::largest_rectangle_area(heights);
    println!("result: {:?}", result);
    ```
````
