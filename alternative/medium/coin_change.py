from typing import List

def coinChange(coins: List[int], amount: int) -> int:
    dp = [amount + 1] * (amount + 1)
    dp[0] = 0

    for a in range(1, amount + 1):
        for c in coins:
            if c <= a:
                dp[a] = min(dp[a], dp[a - c] + 1)
    
    return -1 if dp[amount] > amount else dp[amount]

"""
Algorithm is based on the following recurrence relation:
dp[a] = min(dp[a], dp[a - c] + 1)
where:
  a is the amount of money
  c is the coin value
  dp[a] is the minimum number of coins needed to make up the amount a
  dp[a - c] is the minimum number of coins needed to make up the amount a - c
  + 1 is the coin we are adding to the solution

The algorithm is based on the following observations:
  The minimum number of coins needed to make up the amount a is the minimum
     number of coins needed to make up the amount a - c plus one coin.

Analysis:
Time complexity: O(a * c) = o(n^2)
Space complexity: O(a) = O(n)
"""

assert coinChange([1,2,5], 11) == 3
assert coinChange([2], 3) == -1
assert coinChange([1], 0) == 0