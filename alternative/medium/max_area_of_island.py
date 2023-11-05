from typing import List

def maxAreaOfIsland(grid: List[List[int]]) -> int:
    maxArea = 0
    if not grid:
        return maxArea
    
    ROWS, COLS = len(grid), len(grid[0])

    def dfs(i, j) -> int:
        if i < 0 or j < 0 or i >= ROWS or j >= COLS or grid[i][j] == 0:
            return 0
        grid[i][j] = 0
        res = 1
        res += dfs(i+1, j)
        res += dfs(i-1, j)
        res += dfs(i, j-1)
        res += dfs(i, j+1)
        return res

    for i in range(ROWS):
        for j in range(COLS):
            if grid[i][j] != 0:
                maxArea = max(maxArea, dfs(i, j))
                
    return maxArea

assert maxAreaOfIsland([[0,0,0,0,0,0,0,0]]) == 0

assert maxAreaOfIsland([[0,1,0,0,0,0,0,0],
                        [0,1,0,0,0,0,0,0]]) == 2

assert maxAreaOfIsland([[0,0,1,0,0,0,0,1,0,0,0,0,0],
                        [0,0,0,0,0,0,0,1,1,1,0,0,0],
                        [0,1,1,0,1,0,0,0,0,0,0,0,0],
                        [0,1,0,0,1,1,0,0,1,0,1,0,0],
                        [0,1,0,0,1,1,0,0,1,1,1,0,0],
                        [0,0,0,0,0,0,0,0,0,0,1,0,0],
                        [0,0,0,0,0,0,0,1,1,1,0,0,0],
                        [0,0,0,0,0,0,0,1,1,0,0,0,0]]   ) == 6