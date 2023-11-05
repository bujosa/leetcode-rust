from typing import List

def canPartition(nums: List[int]) -> bool:
    total = sum(nums)
    if total % 2 != 0:
        return False
    target = total // 2
    dp = set()
    dp.add(0)

    for num in nums:
        new_dp = set()
        for i in dp:
            if i + num == target:
                return True
            new_dp.add(i + num)
            new_dp.add(i)
        dp = new_dp
    
    return False

"""
Time complexity: O(n * sum(nums))
Space complexity: O(sum(nums))

Algorithm: Dynamic Programming

Explanation:
1. If the total sum is odd, return False
2. If the total sum is even, we can find a subset that sums to total // 2
3. We can use dynamic programming to find the subset
4. We can use a set to store the sum of all subsets
5. We can iterate through the set and add the current number to each sum
6. If the sum is equal to the target, return True
7. If the sum is not equal to the target, add the sum to the set
8. Repeat steps 5-7 until we reach the end of the array
9. Return False

Example:
[1,5,11,5]
total = 22

"""

assert canPartition([1,5,11,5]) == True
assert canPartition([1,2,3,5]) == False
assert canPartition([1,2]) == False
assert canPartition([1,2,3,4,5,6,7]) == True
assert canPartition([1,2,3,4,5,6,7,8]) == True