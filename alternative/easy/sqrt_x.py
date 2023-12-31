def my_sqrt(x):
    left = 0
    right = x
    ans = -1

    while left <= right:
        mid = left + (right - left) // 2

        if mid > 0 and mid > x // mid:
            right = mid - 1
        else:
            ans = mid
            left = mid + 1

    return ans

# Test cases
assert my_sqrt(0) == 0
assert my_sqrt(1) == 1
assert my_sqrt(4) == 2
assert my_sqrt(8) == 2
assert my_sqrt(9) == 3
assert my_sqrt(10) == 3
assert my_sqrt(30) == 5
assert my_sqrt(2147395599) == 46339