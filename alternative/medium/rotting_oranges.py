from collections import deque
from typing import List

# Amazon Problem
def orangesRotting(grid: List[List[int]]) -> int:
    ROWS, COLS = len(grid), len(grid[0])
    fresh, time = 0, 0
    q = deque()
    for r in range(ROWS):
        for c in range(COLS):
            if grid[r][c] == 1:
                fresh+=1
            if grid[r][c] == 2:
                q.append([r, c])

    directions = [[0, 1], [0, -1], [1, 0], [-1,0]]
    while q and fresh > 0:
        qlen = len(q)
        for _ in range(qlen):
            r, c = q.popleft()

            for dr, dc in directions:
                row, col = dr + r, dc + c
                if (row in range(ROWS) and col in range(COLS) and grid[row][col] == 1):
                    grid[row][col] = 2
                    q.append([row,col])
                    fresh -= 1
                
        time += 1
    
    return time if fresh == 0 else -1

"""
    Algorithm - Breadth First Search
    Time Complexity - O(n)
    Space Complexity - O(n)

    Explanation:
    1. We first check the number of fresh oranges and add the rotten oranges to a queue.
    2. We then iterate through the queue and check the adjacent cells for fresh oranges.
    3. If there are fresh oranges, we add them to the queue and decrement the fresh orange count.
    4. We repeat this process until the queue is empty or there are no fresh oranges left.
    5. If there are no fresh oranges left, we return the time taken to rot all the oranges.
    6. Else, we return -1.
"""

assert orangesRotting([[2,1,1],[1,1,0],[0,1,1]]) == 4
assert orangesRotting([[2,1,1],[0,1,1],[1,0,1]]) == -1