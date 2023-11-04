from typing import List

# This is a solution for a problem frequently asked in Amazon coding interviews.
def maxProduct(nums: List[int]) -> int:
    res = max(nums)
    curMin, curMax = 1, 1

    for n in nums:
        tmp = curMax * n
        curMax = max(n * curMax, n * curMin, n)
        curMin = min(tmp, n * curMin, n)
        res = max(res, curMax, curMin)
    return res


assert maxProduct([2, 3, -2, 4]) == 6
assert maxProduct([-2, 0, -1]) == 0
assert maxProduct([-2]) == -2
assert maxProduct([0, 0, -4, 0, 1, 0, 2]) == 2