def generate(num_rows):
    result = []

    for i in range(num_rows):
        row = []

        for j in range(i + 1):
            if j == 0 or j == i:
                row.append(1)
            else:
                row.append(result[i - 1][j - 1] + result[i - 1][j])

        result.append(row)

    return result

def get_row(row_index):
    result = generate(row_index + 1)

    return result.pop()

# Test cases
assert generate(5) == [
    [1],
    [1, 1],
    [1, 2, 1],
    [1, 3, 3, 1],
    [1, 4, 6, 4, 1]
]
assert get_row(3) == [1, 3, 3, 1]