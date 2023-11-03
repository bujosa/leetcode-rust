from typing import List
import heapq

def lastStoneWeight(stones: List[int]) -> int:
    maxHeap = []
    for stone in stones:
        heapq.heappush(maxHeap, -stone)
    while len(maxHeap) > 1:
        stone1 = -heapq.heappop(maxHeap)
        stone2 = -heapq.heappop(maxHeap)
        if stone1 != stone2:
            heapq.heappush(maxHeap, -(stone1 - stone2))
    return -maxHeap[0] if maxHeap else 0

assert lastStoneWeight([2,7,4,1,8,1]) == 1

# Algorithm - Max Heap
# Time Complexity - O(nlogn)
# Space Complexity - O(n)

# Description

# 1. Initialize a maxHeap
# 2. For each stone in stones, push the negative stone to the maxHeap
# 3. While the length of the maxHeap is greater than 1:
# 4. Pop the maxHeap and assign it to stone1
# 5. Pop the maxHeap and assign it to stone2
# 6. If stone1 is not equal to stone2, push the difference of stone1 and stone2 to the maxHeap
# 7. Return the negative of the maxHeap[0] if the maxHeap is not empty, otherwise return 0
