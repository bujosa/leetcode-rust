class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_word = False

class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        cur = self.root

        for w in word:
            if w not in self.root.children:
                cur.children[w] = TrieNode()
            cur = cur.children[w]
        cur.is_word = True
        

    def search(self, word: str) -> bool:
        cur = self.root

        for w in word:
            if w not in cur.children:
                return False
            cur = cur.children[w]
        return cur.is_word
        

    def startsWith(self, prefix: str) -> bool:
        cur = self.root
        for w in prefix:
            if w not in cur.children:
                return False
            cur = cur.children[w]
        return True
        
assert Trie().insert("apple") == None

trie = Trie()
trie.insert("apple")
assert trie.search("apple") == True
assert trie.search("app") == False
assert trie.startsWith("app") == True
