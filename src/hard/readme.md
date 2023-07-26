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
    let result = hard::trapping_rain_water::trap(height);
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
    let result = hard::largest_rectangle_in_histogram::largest_rectangle_area(heights);
    println!("result: {:?}", result);
    ```
````
