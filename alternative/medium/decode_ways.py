def numDecodings(s: str) -> int:
    dp = { len(s): 1 }

    def dfs(i):
        if i in dp:
            return dp[i]
        if s[i] == '0':
            return 0
        dp[i] = dfs(i+1)
        if i+1 < len(s) and (s[i] == '1' or (s[i] == '2' and s[i+1] < '7')):
            dp[i]+=dfs(i+2)
        return dp[i]
    
    return dfs(0)

# Algorithm idea: Dynamic programming (top-down)
# Time: O(n)

assert numDecodings("12") == 2
assert numDecodings("226") == 3
assert numDecodings("0") == 0
assert numDecodings("06") == 0