from typing import List

def partition(s: str) -> List[List[str]]:
    res = []
    def backtracking(start, path):
        if start == len(s):
            res.append(path)
            return
        for end in range(start + 1, len(s) + 1):
            if is_palindrome(s[start:end]):
                backtracking(end, path + [s[start:end]])
    
    backtracking(0, [])
    return res
    

def is_palindrome(s: str) -> bool:
    return s == s[::-1]

assert partition("aab") == [["a","a","b"],["aa","b"]]