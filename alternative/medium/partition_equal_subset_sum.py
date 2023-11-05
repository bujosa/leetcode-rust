from typing import List

def canPartition(nums: List[int]) -> bool:
    total = sum(nums)
    if total % 2 != 0:
        return False
    target = total // 2
    dp = [False] * (target + 1)
    dp[0] = True
    for num in nums:
        for i in range(target, num - 1, -1):
            dp[i] = dp[i] or dp[i - num]
    return dp[-1]

assert canPartition([1,5,11,5]) == True
assert canPartition([1,2,3,5]) == False
assert canPartition([1,2]) == False
assert canPartition([1,2,3,4,5,6,7]) == True
assert canPartition([1,2,3,4,5,6,7,8]) == True