from typing import List
import heapq
from collections import deque, Counter

def leastInterval(tasks: List[str], n: int) -> int:
    maxHeap = []
    time = 0
    letter_count = {}
    q = deque()

    letter_count = Counter(tasks)
    for value in letter_count.values():
        heapq.heappush(maxHeap, -value)
    
    while maxHeap or q:
        time+=1
        if maxHeap:
            cnt = -heapq.heappop(maxHeap) - 1
            if cnt > 0:
                q.append([-cnt, time + n])
        if q and q[0][1] == time:
            heapq.heappush(maxHeap, q.popleft()[0])
    return time

print(leastInterval(["A","A","A","B","B","B"], 2))
assert leastInterval(["A","A","A","B","B","B"], 2) == 8