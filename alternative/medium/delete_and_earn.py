from typing import List

# This is a solution for a problem frequently asked in Amazon coding interviews.
def deleteAndEarn(nums: List[int]) -> int:
    # Via1 # O(n)
    hash_map = {}
    for num in nums:
        if num in hash_map:
            hash_map[num] += 1
        else:
            hash_map[num] = 1
    # Via2
    # hash_map = collections.Counter(nums) # O(n)
    
    nums = sorted(list(set(nums)))
    earn1, earn2 = 0, 0

    for i in range(len(nums)):
        curEarn = nums[i] * hash_map[nums[i]]

        if i > 0 and nums[i] - nums[i - 1] == 1:
            temp = earn2
            earn2 = max(earn1 + curEarn, earn2)
            earn1 = temp
        else:
            temp = earn2
            earn2 = curEarn + earn2
            earn1 = temp
    
    return earn2


assert deleteAndEarn([3, 4, 2]) == 6
assert deleteAndEarn([2, 2, 3, 3, 3, 4]) == 9