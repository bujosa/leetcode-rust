from typing import List

def combination_sum(candidates: List[int], target: int):
    candidates.sort()
    result = []

    def combination_sum_helper(target, start, current, result):
        if target == 0:
            result.append(list(current))
            return
        for i in range(start, len(candidates)):
            if candidates[i] > target:
                break
            current.append(candidates[i])
            combination_sum_helper(target - candidates[i], i, current, result)
            current.pop()

    combination_sum_helper(target, 0, [], result)
    return result


def test_combination_sum():
    assert combination_sum([2, 3, 6, 7], 7) == [[2, 2, 3], [7]]
    assert combination_sum([2, 3, 5], 8) == [[2, 2, 2, 2], [2, 3, 3], [3, 5]]

test_combination_sum()