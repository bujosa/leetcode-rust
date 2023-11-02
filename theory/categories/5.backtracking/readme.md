# Backtracking

## Theory

### Backtracking

- Backtracking is a general algorithm for finding all (or some) solutions to some computational problems, notably constraint satisfaction problems, that incrementally builds candidates to the solutions, and abandons a candidate ("backtracks") as soon as it determines that the candidate cannot possibly be completed to a valid solution.

- The classic textbook example of the use of backtracking is the eight queens puzzle, that asks for all arrangements of eight chess queens on a standard chessboard so that no queen attacks any other. In the common backtracking approach, the partial candidates are arrangements of k queens in the first k rows of the board, all in different rows and columns. Any partial solution that contains two mutually attacking queens can be abandoned. Backtracking can be applied only for problems which admit the concept of a "partial candidate solution" and a relatively quick test of whether it can possibly be completed to a valid solution. It is useless, for example, for locating a given value in an unordered table. When it is applicable, however, backtracking is often much faster than brute force enumeration of all complete candidates, since it can eliminate many candidates with a single test.

- Backtracking can be applied only for problems which admit the concept of a "partial candidate solution" and a relatively quick test of whether it can possibly be completed to a valid solution. It is useless, for example, for locating a given value in an unordered table. When it is applicable, however, backtracking is often much faster than brute force enumeration of all complete candidates, since it can eliminate many candidates with a single test.

- Backtracking is an important tool for solving constraint satisfaction problems, such as crosswords, verbal arithmetic, Sudoku, and many other puzzles. It is often the most convenient (if not the most efficient) technique for parsing, for the knapsack problem and other combinatorial optimization problems. It is also the basis of the so-called logic programming languages such as Icon, Planner and Prolog.

### Backtracking Applications

- Constraint satisfaction problems
- Crosswords
- Verbal arithmetic
- Sudoku
- Knapsack problem
- Combinatorial optimization problems
- Logic programming languages

### Backtracking Implementation

- Using recursion.

## Problems
- [x] [Subsets](https://leetcode.com/problems/subsets/) | Medium | [Solution](../../../src/medium/subsets.rs) | [Problem Description](../../../src/medium/readme.md#78-subsets)
- [x] [Combination Sum](https://leetcode.com/problems/combination-sum/) | Medium | [Solution](../../../src/medium/combination_sum.rs) | [Problem Description](../../../src/medium/readme.md#39-combination-sum)
- [x] [Permutations](https://leetcode.com/problems/permutations/) | Medium | [Solution](../../../src/medium/permutations.rs) | [Problem Description](../../../src/medium/readme.md#46-permutations)
- [x] [Subsets II](https://leetcode.com/problems/subsets-ii/) | Medium | [Solution](../../../src/medium/subsets_ii.rs) | [Problem Description](../../../src/medium/readme.md#90-subsets-ii)
- [x] [Combination Sum II](https://leetcode.com/problems/combination-sum-ii/) | Medium | [Solution](../../../src/medium/combination_sum_ii.rs) | [Problem Description](../../../src/medium/readme.md#40-combination-sum-ii)
- [x] [Word Search](https://leetcode.com/problems/word-search/) | Medium | [Solution](../../../src/medium/word_search.rs) | [Problem Description](../../../src/medium/readme.md#79-word-search)
- [ ] [Palindrome Partitioning](https://leetcode.com/problems/palindrome-partitioning/) | Medium | [Solution](../../../src/medium/palindrome_partitioning.rs) | [Problem Description](../../../src/medium/readme.md#131-palindrome-partitioning)
- [ ] [Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/) | Medium | [Solution](../../../src/medium/letter_combinations_of_a_phone_number.rs) | [Problem Description](../../../src/medium/readme.md#17-letter-combinations-of-a-phone-number)
- [ ] [N Queens](https://leetcode.com/problems/n-queens/) | Hard | [Solution](../../../src/hard/n_queens.rs) | [Problem Description](../../../src/hard/readme.md#51-n-queens)

Category: `Backtracking`
Created on: 2023-10-31 03:01
Last modified on: 2023-11-01 21:12
Status: In Progress
Author: [David Bujosa](https://github.com/bujosa)