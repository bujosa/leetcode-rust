from typing import List

def maxSubArray(nums: List[int]) -> int:
    maxNumber = nums[0]
    curSum = 0
    
    for n in nums:
        if curSum < 0:
            curSum = 0
        curSum += n
        maxNumber = max(maxNumber, curSum)
    
    return maxNumber
            
"""
    Algorithm - Kadane's Algorithm
    Time Complexity - O(n)
    Space Complexity - O(1)

    Explanation:
    1. Initialize maxNumber and curSum to the first element in the array
    2. Iterate through the array
    3. If curSum is less than 0, set curSum to 0
    4. Add the current element to curSum
    5. Set maxNumber to the max of maxNumber and curSum
    6. Return maxNumber
"""


assert maxSubArray([-2,1,-3,4,-1,2,1,-5,4]) == 6
assert maxSubArray([1]) == 1
assert maxSubArray([5,4,-1,7,8]) == 23
assert maxSubArray([-2,-1]) == -1