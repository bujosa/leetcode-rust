from typing import List

def rob2(nums: List[int]) -> int:
    def helper(nums: List[int]) -> int:
        rob1, rob2 = 0, 0

        for n in nums:
            temp = max(n + rob1, rob2)
            rob1 = rob2
            rob2 = temp

        return rob2

    if len(nums) == 1:
        return nums[0]

    return max(helper(nums[:-1]), helper(nums[1:])) if nums else 0


assert rob2([2,3,2]) == 3
assert rob2([1,2,3,1]) == 4
assert rob2([0]) == 0
assert rob2([1]) == 1
assert rob2([1,2]) == 2
assert rob2([1,2,3]) == 3
