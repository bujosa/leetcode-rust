def length_of_last_word(s):
    count = 0
    last_word = False
    for c in reversed(s):
        if c == ' ':
            if last_word:
                break
        else:
            last_word = True
            count += 1
    return count

# Test cases
assert length_of_last_word("Hello World") == 5
assert length_of_last_word(" ") == 0
assert length_of_last_word("a ") == 1
assert length_of_last_word("a") == 1
assert length_of_last_word("") == 0
assert length_of_last_word("   fly me   to   the moon  ") == 4