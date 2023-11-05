from typing import List

def containsDuplicate(nums: List[int]) -> bool:
    hashset = set()

    for n in nums:
        if n in hashset:
            return True
        hashset.add(n)
    return False

assert containsDuplicate([1,2,3,1]) == True
assert containsDuplicate([1,2,3,4]) == False
assert containsDuplicate([1,1,1,3,3,4,3,2,4,2]) == True