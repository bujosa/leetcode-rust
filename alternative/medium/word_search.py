from typing import List

def exist(board: List[List[str]], word: str) -> bool:
    ROWS, COLS = len(board), len(board[0])
    path = set()
    def dfs(r: int, c: int, i: int):
        if i == len(word):
            return True
        
        if r < 0 or c < 0 or r >= ROWS or c >= COLS or word[i] != board[r][c] or (r,c) in path:
            return False
        
        path.add((r,c))
        path1 = dfs(r + 1, c, i + 1)
        path2 = dfs(r - 1, c, i + 1)
        path3 = dfs(r, c + 1, i + 1)
        path4 = dfs(r, c - 1, i + 1)
        res = path1 or path2 or path3 or path4
        path.remove((r,c))
        return res

    for r in range(ROWS):
        for c in range(COLS):
            if dfs(r, c, 0):
                return True

    return False

assert exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCCED") == True