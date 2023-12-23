def remove_duplicates(nums):
    if len(nums) == 0:
        return 0
    i = 0
    for j in range(1, len(nums)):
        if nums[i] != nums[j]:
            i += 1
            nums[i] = nums[j]
    del nums[i+1:]
    return i + 1

# Test cases
assert remove_duplicates([1,1,2]) == 2
assert remove_duplicates([0,0,1,1,1,2,2,3,3,4]) == 5

nums = [1,1,2]
remove_duplicates(nums)
assert len(nums) == 2