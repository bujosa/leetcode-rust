from typing import List

def numIslands(grid: List[List[str]]) -> int:
    if not grid:
        return 0
    
    ROWS, COLS = len(grid), len(grid[0])

    def dfs(i, j):
        if i < 0 or j < 0 or i >= ROWS or j >= COLS or grid[i][j] == '0':
            return
        grid[i][j] = '0'
        dfs(i+1, j)
        dfs(i-1, j)
        dfs(i, j-1)
        dfs(i, j+1)

    islands = 0
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            if grid[i][j] != '0':
                dfs(i, j)
                islands += 1
    return islands

"""
    Algorithm - Breadth First Search
    1. Iterate through the grid
    2. If the current element is 1, increment the count and call bfs
    3. In bfs, mark the current element as 0 and call bfs on all the adjacent elements
    4. Return the count
"""

assert numIslands([["1","1","1","1","0"], 
                   ["1","1","0","1","0"], 
                   ["1","1","0","0","0"], 
                   ["0","0","0","0","0"]]) == 1

assert numIslands([["1","1","0","0","0"],
                     ["1","1","0","0","0"],
                     ["0","0","1","0","0"],
                     ["0","0","0","1","1"]]) == 3
