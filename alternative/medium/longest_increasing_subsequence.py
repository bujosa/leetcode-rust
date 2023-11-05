from typing import List

def lengthOfLIS( nums: List[int]) -> int:
    if not nums:
        return 0
    dp = [1] * len(nums)
    # Dynamic programming - Bottom up approach - O(n^2)
    for i in range(len(nums)):
        for j in range(i):
            if nums[i] > nums[j]:
                dp[i] = max( dp[i], dp[j] + 1)
    return max(dp)

assert lengthOfLIS([10,9,2,5,3,7,101,18]) == 4
assert lengthOfLIS([0,1,0,3,2,3]) == 4