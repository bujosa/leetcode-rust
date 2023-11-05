from typing import List

def solve(board: List[List[str]]) -> None:
    """
    Do not return anything, modify board in-place instead.
    """
    ROWS, COLS = len(board), len(board[0])
    def dfs(r, c):
        if r < 0 or c < 0 or r == ROWS or c == COLS or board[r][c] != "O":
            return
        board[r][c] = "#"
        dfs(r - 1, c)
        dfs(r + 1, c)
        dfs(r, c - 1)
        dfs(r, c + 1)

    for r in range(ROWS):
        dfs(r, 0)
        dfs(r, COLS - 1)

    for c in range(COLS):
        dfs(0, c)
        dfs(ROWS - 1, c)

    for r in range(ROWS):
        for c in range(COLS):
            if board[r][c] == "#":
                board[r][c] = "O"
            elif board[r][c] == "O":
                board[r][c] = "X"

"""
     Algorithm - DFS
     Time complexity: O(m*n)
     Space complexity: O(m*n) for the call stack
""" 

board = [["X","X","X","X"], ["X","O","O","X"], ["X","X","O","X"], ["X","O","X","X"]]
solve(board)
print(board)
assert board == [["X","X","X","X"], ["X","X","X","X"], ["X","X","X","X"], ["X","O","X","X"]]