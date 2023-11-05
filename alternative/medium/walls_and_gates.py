from collections import deque
from math import inf
from typing import List

# Amazon Problem Wall and Gates
def wallsAndGates(rooms: List[List[int]]) -> None:
    ROWS, COLS = len(rooms), len(rooms[0])
    q = deque()

    for r in range(ROWS):
        for c in range(COLS):
            if rooms[r][c] == 0:
                q.append([r, c])

    directions = [[0, 1], [0, -1], [1, 0], [-1,0]]
    time = 1
    while q:
        qlen = len(q)
        for _ in range(qlen):
            r, c = q.popleft()
            for dr, dc in directions:
                row, col = dr + r, dc + c
                if(row in range(ROWS) and col in range(COLS) and rooms[row][col] == inf):
                    rooms[row][col] = time
                    q.append([row, col])

        time+=1

rooms = [[inf,-1,0,inf],[inf,inf,inf,-1],[inf,-1,inf,-1],[0,-1,inf,inf]]
wallsAndGates(rooms)
print(rooms)
assert rooms == [[3,-1,0,1],[2,2,1,-1],[1,-1,2,-1],[0,-1,3,4]]
rooms2 = [[inf,-1,0,inf],
          [inf,inf,inf,-1],
          [inf,-1,inf,-1],
          [-1,-1,inf,inf]]
wallsAndGates(rooms2) 
print(rooms2)
assert rooms2  == [[4, -1, 0, 1], [3, 2, 1, -1], [4, -1, 2, -1], [-1, -1, 3, 4]]
rooms3 = [[inf,-1,0,inf],
          [-1,-1,inf,-1],
          [inf,-1,inf,-1],
          [-1,-1,inf,inf]]
wallsAndGates(rooms3)
print(rooms3)
assert rooms3 == [[inf, -1, 0, 1], [-1, -1, 1, -1], [inf, -1, 2, -1], [-1, -1, 3, 4]]