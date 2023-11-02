from typing import List

tn_representations = {
    "2": "abc",
    "3": "def",
    "4": "ghi",
    "5": "jkl",
    "6": "mon",
    "7": "pqrs",
    "8": "tuv",
    "9": "wxyz"
}

def letterCombinations(digits: str) -> List[str]:
    res = []

    if len(digits) == 0:
        return res

    def backtracking(i, curStr):
        if len(curStr) == len(digits):
            res.append(curStr)
            return
        
        for c in tn_representations[digits[i]]:
            backtracking(i+1, curStr + c)


    backtracking(0, "")
    return res


assert letterCombinations("23") == ["ad","ae","af","bd","be","bf","cd","ce","cf"]