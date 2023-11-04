from typing import List
import heapq

# Algorithm - Max Heap
# Time Complexity - O(nlogn)
# Space Complexity - O(n)
def findKthLargest(nums: List[int], k: int) -> int:
    max_heap = []

    for n in nums:
        heapq.heappush(max_heap, -n)
    
    while k > 1:
        heapq.heappop(max_heap)
        k -= 1
        
    return -heapq.heappop(max_heap)

# Algorithm - Quick Select
# Time Complexity - O(n)
# Space Complexity - O(1)
def findKthLargestV2(nums: List[int], k: int) -> int:
    k = len(nums) - k
    left, right = 0, len(nums) - 1

    while left < right:
        pivot = partition(nums, left, right)
        if pivot < k:
            left = pivot + 1
        elif pivot > k:
            right = pivot - 1
        else:
            break

    return nums[k]

def partition(nums: List[int], left: int, right: int) -> int:
    pivot = left
    for i in range(left, right):
        if nums[i] <= nums[right]:
            nums[i], nums[pivot] = nums[pivot], nums[i]
            pivot += 1

    nums[pivot], nums[right] = nums[right], nums[pivot]

    return pivot
    
assert findKthLargest([3,2,1,5,6,4], 2) == 5
assert findKthLargestV2([3,2,1,5,6,4], 2) == 5
assert findKthLargest([3,2,3,1,2,4,5,5,6], 4) == 4
assert findKthLargestV2([3,2,3,1,2,4,5,5,6], 4) == 4