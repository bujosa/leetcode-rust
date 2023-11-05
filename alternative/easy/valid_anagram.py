

def isAnagram(s: str, t: str) -> bool:

    if len(s) != len(t):
        return False

    first = dict()
    second = dict()

    for c in s:
        first[c] = first.get(c, 0) + 1
    
    for c in t:
        second[c] = second.get(c, 0) + 1

    return first == second


assert isAnagram("anagram", "nagaram") == True
assert isAnagram("rat", "car") == False
assert isAnagram("a", "ab") == False
assert isAnagram("ab", "a") == False
assert isAnagram("ab", "ba") == True