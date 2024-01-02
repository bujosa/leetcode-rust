from collections import deque

def max_sliding_window(nums, k):
    result = []
    window = deque()

    for i in range(len(nums)):
        # Remove the first element if it is out of the window
        if window and window[0] <= i - k:
            window.popleft()

        # Remove all elements smaller than the current one
        while window and nums[window[-1]] <= nums[i]:
            window.pop()

        # Add the current element
        window.append(i)

        # Add the maximum to the result
        if i >= k - 1:
            result.append(nums[window[0]])

    return result

# Test cases
assert max_sliding_window([1, 3, -1, -3, 5, 3, 6, 7], 3) == [3, 3, 5, 5, 6, 7]
assert max_sliding_window([1], 1) == [1]
assert max_sliding_window([1, -1], 1) == [1, -1]
assert max_sliding_window([9, 11], 2) == [11]