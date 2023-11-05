from typing import List

def jump(nums: List[int]) -> int:
    minJump = 0
    l = r = 0
    
    while r < len(nums) - 1:
        farthest = 0
        
        for i in range(l, r + 1):
            farthest = max(farthest, i + nums[i])
        
        l = r + 1
        r = farthest
        minJump += 1
    
    return minJump

# You always can reach the last index.
assert jump([2, 3, 1, 1, 4]) == 2
assert jump([2, 3, 0, 1, 4]) == 2
assert jump([2, 3, 0, 1, 0]) == 2
assert jump([4, 3, 0, 1, 0]) == 1