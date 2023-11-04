def longestPalindrome(s: str) -> str:
    res = ""

    for i in range(len(s)):
        for l, r in ((i,i), (i,i+1)):
            while l >= 0 and r < len(s) and s[l] == s[r]:
                if (r - l + 1) > len(res):
                    res = s[l:r + 1]
                l -= 1
                r += 1

    return res

# Algorithm idea: Expand around center
# Time: O(n^2)
# Space: O(1)

assert longestPalindrome("babad") == "bab"
assert longestPalindrome("cbbd") == "bb"
assert longestPalindrome("a") == "a"
assert longestPalindrome("ac") == "a"