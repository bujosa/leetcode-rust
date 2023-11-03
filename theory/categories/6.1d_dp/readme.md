# 1-D DP

## Theory

### Dynamic Programming

Dynamic programming is a method for solving a complex problem by breaking it down into a collection of simpler subproblems, solving each of those subproblems just once, and storing their solutions using a memory-based data structure (array, map, etc). Each of the subproblem solutions is indexed in some way, typically based on the values of its input parameters, so as to facilitate its lookup. So the next time the same subproblem occurs, instead of recomputing its solution, one simply looks up the previously computed solution, thereby saving computation time.

Dynamic programming algorithms are used for optimization (for example, finding the shortest path between two points, or the fastest way to multiply many matrices). A dynamic programming algorithm will examine the previously solved subproblems and will combine their solutions to give the best solution for the given problem. In comparison, a greedy algorithm treats the solution as some sequence of steps and picks the locally optimal choice at each step. Using a greedy algorithm does not guarantee an optimal solution, because picking locally optimal choices may result in a bad global solution, but it is often faster to calculate. Some greedy algorithms (such as Kruskal's or Prim's for minimum spanning trees) are however proven to lead to the optimal solution.

### Dynamic Programming vs Divide and Conquer

Dynamic programming and divide and conquer are similar. Both are recursive and both solve sub-problems. However, divide and conquer combines solutions to sub-problems to form solutions to larger problems, whereas dynamic programming combines solutions to sub-problems to form solutions to even larger problems. Dynamic programming is used when the sub-problems are not independent, whereas divide and conquer is used when the sub-problems are independent.

### Dynamic Programming vs Greedy

Dynamic programming and greedy algorithms are similar. Both are optimization methods and both solve sub-problems. However, greedy algorithms make the optimal choice at each step as it attempts to find the overall optimal way to solve the entire problem. Dynamic programming, on the other hand, does not necessarily make the optimal choice at each step. It attempts to solve the sub-problems first and then uses the solutions to the sub-problems to solve the overall problem. Dynamic programming is used when the sub-problems are not independent, whereas greedy algorithms are used when the sub-problems are independent.


## Problems

- [ ] [Climbing Stairs](https://leetcode.com/problems/climbing-stairs/) | Easy | [Solution](../../../src/easy/climbing_stairs.rs) | [Problem Description](../../../src/easy/readme.md#70-climbing-stairs)
- [ ] [House Robber](https://leetcode.com/problems/house-robber/) | Medium | [Solution](../../../src/medium/house_robber.rs) | [Problem Description](../../../src/medium/readme.md#198-house-robber)
- [ ] [House Robber II](https://leetcode.com/problems/house-robber-ii/) | Medium | [Solution](../../../src/medium/house_robber_ii.rs) | [Problem Description](../../../src/medium/readme.md#213-house-robber-ii)
- [ ] [Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/) | Medium | [Solution](../../../src/medium/longest_palindromic_substring.rs) | [Problem Description](../../../src/medium/readme.md#5-longest-palindromic-substring)
- [ ] [Palindromic Substrings](https://leetcode.com/problems/palindromic-substrings/) | Medium | [Solution](../../../src/medium/palindromic_substrings.rs) | [Problem Description](../../../src/medium/readme.md#647-palindromic-substrings)
- [ ] [Decode Ways](https://leetcode.com/problems/decode-ways/) | Medium | [Solution](../../../src/medium/decode_ways.rs) | [Problem Description](../../../src/medium/readme.md#91-decode-ways)
- [ ] [Coin Change](https://leetcode.com/problems/coin-change/) | Medium | [Solution](../../../src/medium/coin_change.rs) | [Problem Description](../../../src/medium/readme.md#322-coin-change)
- [ ] [Maximum Product Subarray](https://leetcode.com/problems/maximum-product-subarray/) | Medium | [Solution](../../../src/medium/maximum_product_subarray.rs) | [Problem Description](../../../src/medium/readme.md#152-maximum-product-subarray)
- [ ] [Word Break](https://leetcode.com/problems/word-break/) | Medium | [Solution](../../../src/medium/word_break.rs) | [Problem Description](../../../src/medium/readme.md#139-word-break)
- [ ] [Longest Increasing Subsequence](https://leetcode.com/problems/longest-increasing-subsequence/) | Medium | [Solution](../../../src/medium/longest_increasing_subsequence.rs) | [Problem Description](../../../src/medium/readme.md#300-longest-increasing-subsequence)
- [ ] [Partition Equal Subset Sum](https://leetcode.com/problems/partition-equal-subset-sum/) | Medium | [Solution](../../../src/medium/partition_equal_subset_sum.rs) | [Problem Description](../../../src/medium/readme.md#416-partition-equal-subset-sum)