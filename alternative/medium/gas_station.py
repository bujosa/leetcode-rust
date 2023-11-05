from typing import List

def canCompleteCircuit(gas: List[int], cost: List[int]) -> int:
    if sum(gas) < sum(cost):
        return -1

    curSum, res = 0, 0
    for i in range(0, len(gas)):
        curSum += gas[i] - cost[i]
        if curSum < 0:
            curSum = 0
            res = i + 1
    
    return res

assert canCompleteCircuit([5,8,2,8], [6,5,6,6]) == 3
assert canCompleteCircuit([1, 2, 3, 4, 5], [3, 4, 5, 1, 2]) == 3
assert canCompleteCircuit([2, 3, 4], [3, 4, 3]) == -1
assert canCompleteCircuit([3, 1, 1], [1, 2, 2]) == 0
assert canCompleteCircuit([1, 2, 3], [1, 2, 3]) == 0
assert canCompleteCircuit([2, 3, 4], [3, 4, 4]) == -1