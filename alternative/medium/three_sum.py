from typing import List

def threeSum(nums: List[int]) -> List[List[int]]:
    sortedArray = sorted(nums) # O(nlogn)
    res = []

    # O(n^2)
    for i in range(len(sortedArray)):
        if i > 0 and sortedArray[i] == sortedArray[i - 1]:
            continue
        left = i + 1
        right = len(sortedArray) - 1
        while left < right:
            sum = sortedArray[i] + sortedArray[left] + sortedArray[right]
            if sum == 0:
                res.append([sortedArray[i], sortedArray[left], sortedArray[right]])
                left += 1
                right -= 1
                while left < right and sortedArray[left] == sortedArray[left - 1]:
                    left += 1
                while left < right and sortedArray[right] == sortedArray[right + 1]:
                    right -= 1
            elif sum < 0:
                left += 1
            else:
                right -= 1

    return res


assert threeSum([-1,0,1,2,-1,-4]) == [[-1,-1,2],[-1,0,1]]
assert threeSum([]) == []
assert threeSum([0]) == []
assert threeSum([0,0,0]) == [[0,0,0]]