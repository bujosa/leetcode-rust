def subsets_with_dup(nums):
    res = []
    nums.sort()
    def backtrack(i, subset): 
        res.append(subset)
        for j in range(i, len(nums)):
            if j > i and nums[j] == nums[j - 1]:
                continue
            else:
                backtrack(j + 1, subset + [nums[j]])
    backtrack(0, [])
    return res



assert subsets_with_dup([1, 2, 2]) == [[], [1], [1, 2], [1, 2, 2], [2], [2, 2]]