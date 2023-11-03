from typing import List

class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_word = False

class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        cur = self.root

        for c in word:
            if c not in cur.children:
                cur.children[c] = TrieNode()
            cur = cur.children[c]
        cur.is_word = True
        

    def search(self, word: str) -> bool:
        cur = self.root

        for c in word:
            if c not in cur.children:
                return False
            cur = cur.children[c]
        return cur.is_word
        

    def startsWith(self, prefix: str) -> bool:
        cur = self.root
        for c in prefix:
            if c not in cur.children:
                return False
            cur = cur.children[c]
        return True

def findWords( board: List[List[str]], words: List[str]) -> List[str]:
    trie = Trie()

    for word in words:
        trie.insert(word)

    res = []
    ROWS, COLS = len(board), len(board[0])
    path = set()

    def dfs(r: int, c: int, word: str):
        if r < 0 or c < 0 or r >= ROWS or c >= COLS or (r,c) in path: 
            return

        word = word + board[r][c]
        if trie.startsWith(word) == False:
            return

        path.add((r,c))
        if trie.search(word):
            res.append(word)

        dfs(r + 1, c, word)
        dfs(r - 1, c, word)
        dfs(r, c + 1, word)
        dfs(r, c - 1, word)
        path.remove((r,c))
        return res

    for r in range(ROWS):
        for c in range(COLS):
            dfs(r, c, "")

    return res

print(findWords([["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], ["oath","pea","eat","rain"]))

assert findWords([["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], ["oath","pea","eat","rain"]) == ["oath","eat"]