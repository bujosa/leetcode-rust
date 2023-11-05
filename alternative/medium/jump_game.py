from typing import List

def canJump(nums: List[int]) -> bool:
    goal = len(nums) - 1

    for i in range(len(nums) - 1, -1, -1):
        if i + nums[i] >= goal:
            goal = i
    
    return True if goal == 0 else False

"""
    Algorithm 1: Greedy
    - Start from the end of the array
    - Keep track of the goal
    - If the current index + the value at the current index is greater than
      or equal to the goal, update the goal to the current index
    - If the goal is 0, return True
    - Else, return False
    - O(n) time complexity
    - O(1) space complexity

"""

assert canJump([2, 3, 1, 1, 4]) == True
assert canJump([3, 2, 1, 0, 4]) == False
assert canJump([0]) == True
assert canJump([1, 0, 1, 0]) == False
assert canJump([1, 1, 1, 0]) == True