def find_binary_gap(n):
    max_gap = 0
    current_gap = 0
    start_counting = False
    num = n
    while num > 0:
        if num & 1 == 1:
            if not start_counting:
                start_counting = True
            else:
                max_gap = max(max_gap, current_gap)
            current_gap = 0
        elif start_counting:
            current_gap += 1
        num >>= 1
    return max_gap

# Test cases
assert find_binary_gap(9) == 2
assert find_binary_gap(529) == 4
assert find_binary_gap(20) == 1
assert find_binary_gap(15) == 0
assert find_binary_gap(32) == 0
assert find_binary_gap(1041) == 5