from typing import List

def permute(nums: List[int]) -> List[List[int]]:
    if len(nums) == 1:
        return [nums]
    result = []
    for i in range(len(nums)):
        for j in permute(nums[:i] + nums[i + 1:]):
            result.append([nums[i]] + j)
    return result


assert permute([1, 2, 3]) == [
    [1, 2, 3],
    [1, 3, 2],
    [2, 1, 3],
    [2, 3, 1],
    [3, 1, 2],
    [3, 2, 1],
]