from typing import List

def combinationSum2(candidates: List[int], target: int) -> List[List[int]]:
    candidates.sort()
    result = []
    def backtracking(target, start, current):
        if target == 0:
            result.append(list(current))
            return
        for i in range (start, len(candidates)):
            if i > start and candidates[i] == candidates[i - 1]:
                continue
            if candidates[i] > target or target - candidates[i] < 0:
                continue
            current.append(candidates[i])
            backtracking(target - candidates[i], i + 1, current)
            current.pop()
            
    backtracking(target, 0, [])
    return result

print(combinationSum2([10,1,2,7,6,1,5], 8))
assert combinationSum2([10,1,2,7,6,1,5], 8) == [[1,1,6],[1,2,5],[1,7],[2,6]]