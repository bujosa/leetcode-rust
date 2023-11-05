def isPalindrome(s: str) -> bool:
    new_s = ""
    for c in s:
        if c.isalnum():
            new_s += c.lower()

    return new_s == new_s[::-1]

assert isPalindrome("A man, a plan, a canal: Panama") == True
assert isPalindrome("A b") == False
assert isPalindrome("A") == True
assert isPalindrome("") == True
assert isPalindrome(" ") == True
assert isPalindrome("a") == True