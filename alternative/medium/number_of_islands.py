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
    Algorithm - DFS
    Time Complexity - O(M*N)
    Space Complexity - O(M*N)
    where M is the number of rows and N is the number of columns

    Description:
    1. Iterate through each cell in the grid.
    2. If the cell is a '1', then it is the top-left corner of an island.
    3. Perform a DFS on the cell to mark all the cells in the island as '0'.
    4. Increment the number of islands and continue the search.
"""

assert numIslands([["1","1","1","1","0"], 
                   ["1","1","0","1","0"], 
                   ["1","1","0","0","0"], 
                   ["0","0","0","0","0"]]) == 1

assert numIslands([["1","1","0","0","0"],
                     ["1","1","0","0","0"],
                     ["0","0","1","0","0"],
                     ["0","0","0","1","1"]]) == 3
