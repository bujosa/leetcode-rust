# Heap / Priority Queue

## Theory

### Heap

A heap is a complete binary tree that satisfies the heap property. The heap property states that for every node `x` with parent `p`, the key in `p` is smaller than or equal to the key in `x`.

A heap is a complete binary tree, which means that all levels of the tree, except possibly the last one (deepest) are fully filled, and, if the last level of the tree is not complete, the nodes of that level are filled from left to right.

#### Heap Implementation

A heap can be implemented using an array or a linked list.

#### Heap Time Complexity

- Access: O(n)
- Search: O(n)
- Insertion: O(1)
- Deletion: O(1)

#### Heap Space Complexity

- O(n)

#### Heap Applications

- Heap Sort
- Priority Queue
- Graph Algorithms
- Selection Algorithms
- Event-driven Simulation

### Priority Queue

A priority queue is an abstract data type similar to a regular queue or stack data structure in which each element additionally has a "priority" associated with it. In a priority queue, an element with high priority is served before an element with low priority. If two elements have the same priority, they are served according to their order in the queue.

While priority queues are often implemented with heaps, they are conceptually distinct from heaps. A priority queue is an abstract concept like "a list" or "a map"; just as a list can be implemented with a linked list or an array, a priority queue can be implemented with a heap or a variety of other methods such as an unordered array.


#### Priority Queue Implementation

A priority queue can be implemented using a heap.

#### Priority Queue Time Complexity

- Access: O(n)
- Search: O(n)
- Insertion: O(1)
- Deletion: O(1)

#### Priority Queue Space Complexity

- O(n)

#### Priority Queue Applications

- Dijkstra's algorithm
- Prim's algorithm
- Huffman coding
- Best-first search algorithm
- A* search algorithm

## Problems

- [ ] [Kth Largest Element in a Stream](https://leetcode.com/problems/kth-largest-element-in-a-stream/) | Easy | [Solution](../../../src/easy/kth_largest_element_in_a_stream.rs) | [Problem Description](../../../src/easy/readme.md#703-kth-largest-element-in-a-stream)
- [ ] [Last Stone Weight](https://leetcode.com/problems/last-stone-weight/) | Easy | [Solution](../../../src/easy/last_stone_weight.rs) | [Problem Description](../../../src/easy/readme.md#1046-last-stone-weight)
- [ ] [K Closest Points to Origin](https://leetcode.com/problems/k-closest-points-to-origin/) | Medium | [Solution](../../../src/medium/k_closest_points_to_origin.rs) | [Problem Description](../../../src/medium/readme.md#973-k-closest-points-to-origin)
- [ ] [Kth Largest Element in an Array](https://leetcode.com/problems/kth-largest-element-in-an-array/) | Medium | [Solution](../../../src/medium/kth_largest_element_in_an_array.rs) | [Problem Description](../../../src/medium/readme.md#215-kth-largest-element-in-an-array)
- [ ] [Task Scheduler](https://leetcode.com/problems/task-scheduler/) | Medium | [Solution](../../../src/medium/task_scheduler.rs) | [Problem Description](../../../src/medium/readme.md#621-task-scheduler)
- [ ] [Find Median from Data Stream](https://leetcode.com/problems/find-median-from-data-stream/) | Hard | [Solution](../../../src/hard/find_median_from_data_stream.rs) | [Problem Description](../../../src/hard/readme.md#295-find-median-from-data-stream)

Category: `Heap / Priority Queue`
Created on: 2023-11-02 21:12
Last modified on: 2023-11-02 21:12
Status: In Progress
Author: [David Bujosa](https://github.com/bujosa)