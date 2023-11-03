from typing import List
import heapq

def findKthLargest(nums: List[int], k: int) -> int:
    max_heap = []

    for n in nums:
        heapq.heappush(max_heap, -n)
    
    while k > 1:
        heapq.heappop(max_heap)
        k -= 1
        
    return -heapq.heappop(max_heap)

# Algorithm - Max Heap
# Time Complexity - O(nlogn)
# Space Complexity - O(n)

assert findKthLargest([3,2,1,5,6,4], 2) == 5
assert findKthLargest([3,2,3,1,2,4,5,5,6], 4) == 4