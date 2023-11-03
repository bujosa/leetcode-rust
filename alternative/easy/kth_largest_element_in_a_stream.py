class KthLargest:
    def __init__(self, k: int, nums: List[int]):
        self.minHeap. self.k = nums, k
        heapq.heapify(self.minHeap)
        while len(self.minHeap) > k:
            heapq.heappop(self.minHeap)
        
    def add(self, val: int) -> int:
        heapq.heappush(self.minHeap, val)
        if len(self.minHeap) > self.k:
            heapq.heappop(self.minHeap)
        return self.minHeap[0]

# Algorithm - Min Heap
# Time Complexity - O(nlogk)
# Space Complexity - O(k)

# Description
# 1. Initialize a minHeap and a k
# 2. Heapify the minHeap
# 3. While the length of the minHeap is greater than k, pop the minHeap
# 4. Add the value to the minHeap
# 5. If the length of the minHeap is greater than k, pop the minHeap
# 6. Return the minHeap[0]

# Test
assert KthLargest(3, [4, 5, 8, 2]).add(3) == 4

# Your KthLargest object will be instantiated and called as such:
# obj = KthLargest(k, nums)
# param_1 = obj.add(val)