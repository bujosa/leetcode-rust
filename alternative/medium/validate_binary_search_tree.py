from typing import List
from collections import deque

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def is_valid_bst(root: 'TreeNode') -> int:
    def helper(node, lower, upper):
        if not node:
            return True
        if node.val <= lower or node.val >= upper:
            return False
        return helper(node.left, lower, node.val) and helper(node.right, node.val, upper)
    return helper(root, float('-inf'), float('inf'))

def test_is_valid_bst():
    t1 = TreeNode(2)
    t2 = TreeNode(1)
    t3 = TreeNode(3)
    t1.left = t2
    t1.right = t3
    print(is_valid_bst(t1))
    assert is_valid_bst(t1) == True

test_is_valid_bst()
