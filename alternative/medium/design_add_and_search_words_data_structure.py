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

class WordDictionary:
    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.trie = Trie()

        

    def addWord(self, word: str) -> None:
        """
        Adds a word into the data structure.
        """
        self.trie.insert(word)
        

    def search(self, word: str) -> bool:
        """
        Returns if the word is in the data structure. 
        A word could contain the dot character '.' to represent any one letter.
        """
        
        def helper(word, node):
            cur = node
            for i, c in enumerate(word):
                if c == '.':
                    for child in cur.children.values():
                        if helper(word[i+1:], child):
                            return True
                    return False
                if c not in cur.children:
                    return False
                cur = cur.children[c]
            return cur.is_word
        
        return helper(word, self.trie.root)
        


# Your WordDictionary object will be instantiated and called as such:
# obj = WordDictionary()
# obj.addWord(word)
# param_2 = obj.search(word)