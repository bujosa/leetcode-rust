from typing import List

# Best Time to Buy and Sell Stock
def maxProfit(prices: List[int]) -> int:
    maxProfit = 0
    l, r = 0, 1

    while r < len(prices):
        if prices[l] < prices[r]:
            profit = prices[r] - prices[l]
            maxProfit = max(profit, maxProfit)
        else:
            l +=1
        
        r +=1
        
    return maxProfit


assert maxProfit([7,1,5,3,6,4]) == 5
assert maxProfit([7,6,4,3,1]) == 0