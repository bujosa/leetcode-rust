from typing import List

def wordBreak(s: str, wordDict: List[str]) -> bool:
    # Convert the wordDict to a set for faster lookup
    wordSet = set(wordDict)
    # Create a dp array of size s + 1
    dp = [False] * (len(s) + 1)
    # Set the first element of the dp array to True
    dp[0] = True

    # Iterate through the dp array Bottom Up
    for i in range(1, len(dp)):
        # Iterate through the dp array again
        for j in range(i):
            # If the dp[j] is True and the substring from j to i is in the wordSet
            if dp[j] and s[j:i] in wordSet:
                # Set the dp[i] to True
                dp[i] = True
                # Break out of the loop
                break
        
    # Return the last element of the dp array
    return dp[-1]

assert wordBreak("leetcode", ["leet", "code"]) == True
assert wordBreak("leetleet", ["leet", "code"]) == True
assert wordBreak("applepenapple", ["apple", "pen"]) == True
assert wordBreak("catsandog", ["cats", "dog", "sand", "and", "cat"]) == False        