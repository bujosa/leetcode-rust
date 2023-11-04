
def countSubstrings(s: str) -> int:
    count = 0
    
    for i in range(len(s)):
        for l, r in ((i, i), (i, i+1)):
            while l >= 0 and r < len(s) and s[l] == s[r]:
                count+=1
                l-=1
                r+=1

    return count

# Algorithm idea: Expand around center
# Time: O(n^2)
# Space: O(1)

assert countSubstrings("abc") == 3      
assert countSubstrings("aaa") == 6
assert countSubstrings("abba") == 6
assert countSubstrings("a") == 1
assert countSubstrings("ac") == 2