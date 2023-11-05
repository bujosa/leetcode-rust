from typing import List

def twoSum(numbers: List[int], target: int) -> List[int]:
    l, r = 0, len(numbers) - 1
    while l < r:
        currentSum = numbers[l] + numbers[r]
        if currentSum > target:
            r -= 1
        elif currentSum < target:
            l += 1
        else:
            return [l+1, r+1]

    return []

assert twoSum([2,7,11,15], 9) == [1,2]
assert twoSum([2,3,4], 6) == [1,3]
assert twoSum([-1,0], -1) == [1,2]

