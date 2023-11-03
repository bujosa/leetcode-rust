from typing import List
import heapq

def findKthLargest(nums: List[int], k: int) -> int:
    priority_queue = []

    for n in nums:
        heapq.heappush(priority_queue, -n)
    
    res = 0
    while k > 0:
        res = -heapq.heappop(priority_queue)
        k -= 1
    return res

assert findKthLargest([3,2,1,5,6,4], 2) == 5
assert findKthLargest([3,2,3,1,2,4,5,5,6], 4) == 4