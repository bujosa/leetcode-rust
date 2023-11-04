from typing import List

def minCostClimbingStairs(cost: List[int]) -> int:
    cost.append(0)

    for i in range(len(cost) - 3, -1, -1):
        cost[i] += min(cost[i + 1], cost[i + 2])

    return min(cost[0], cost[1])


print(minCostClimbingStairs([10, 15, 20]))
assert minCostClimbingStairs([10, 15, 20]) == 15
assert minCostClimbingStairs([1, 100, 1, 1, 1, 100, 1, 1, 100, 1]) == 6
