from typing import List

def subsets(nums: List[int]) -> List[List[int]]:
    def backtrack(start: int, current: List[int]) -> None:
        result.append(list(current))
        for i in range(start, len(nums)):
            current.append(nums[i])
            backtrack(i + 1, current)
            current.pop()

    result = []
    backtrack(0, [])
    return result

# Create function to test for validate if the result have the same subsets as expected doest not matter the order
def validate(expected: List[List[int]], result: List[List[int]]) -> bool:
    if len(expected) != len(result):
        return False
    for subset in expected:
        if subset not in result:
            return False
    return True

assert validate([[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]], subsets([1, 2, 3]))