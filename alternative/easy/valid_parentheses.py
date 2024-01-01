def is_valid(s):
    stack = []
    for c in s:
        if c in "([{":
            stack.append(c)
        elif c == ')' and (not stack or stack.pop() != '('):
            return False
        elif c == ']' and (not stack or stack.pop() != '['):
            return False
        elif c == '}' and (not stack or stack.pop() != '{'):
            return False
    return not stack

# Test cases
assert is_valid("()") == True
assert is_valid("()[]{}") == True
assert is_valid("(]") == False
assert is_valid("([)]") == False
assert is_valid("{[]}") == True