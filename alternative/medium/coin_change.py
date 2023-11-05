from typing import List

def coinChange(coins: List[int], amount: int) -> int:
    dp = [amount + 1] * (amount + 1)
    dp[0] = 0

    for a in range(1, amount + 1):
        for c in coins:
            if c <= a:
                dp[a] = min(dp[a], dp[a - c] + 1)
    
    return -1 if dp[amount] > amount else dp[amount]


assert coinChange([1,2,5], 11) == 3
assert coinChange([2], 3) == -1
assert coinChange([1], 0) == 0