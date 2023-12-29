def two_sum(nums, target):
    map = {}

    for i, num in enumerate(nums):
        complement = target - num

        if complement in map:
            return [map[complement], i]

        map[num] = i

    return []

# Test cases
assert two_sum([2, 7, 11, 15], 9) == [0, 1]
assert two_sum([3, 2, 4], 6) == [1, 2]
assert two_sum([3, 3], 6) == [0, 1]