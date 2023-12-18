def plus_one(digits):
    i = len(digits) - 1
    while i >= 0:
        if digits[i] == 9:
            digits[i] = 0
            if i == 0:
                digits.insert(0, 1)
                break
            i -= 1
        else:
            digits[i] += 1
            break
    return digits

# Test cases
assert plus_one([1, 2, 3]) == [1, 2, 4]
assert plus_one([4, 3, 2, 1]) == [4, 3, 2, 2]
assert plus_one([0]) == [1]
assert plus_one([9]) == [1, 0]
assert plus_one([9, 9]) == [1, 0, 0]
assert plus_one([9, 9, 9]) == [1, 0, 0, 0]
assert plus_one([9, 9, 9, 9]) == [1, 0, 0, 0, 0]
assert plus_one([9, 9, 9, 9, 9]) == [1, 0, 0, 0, 0, 0]