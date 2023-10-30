from typing import List
from collections import deque

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def good_nodes(root: 'TreeNode') -> int:
    res = 0
    q = deque([(root, float('-inf'))])

    while q:
        node, maxVal = q.popleft()
        if node:
            if node.val >= maxVal:
                res += 1
            q.append((node.left, max(node.val, maxVal)))
            q.append((node.right, max(node.val, maxVal)))
    return res

def test_good_nodes():
    t1 = TreeNode(3)
    t2 = TreeNode(3)
    t3 = TreeNode(4)
    t4 = TreeNode(2)
    t1.left = t2
    t2.right = t4
    t2.left = t3
    result = 3
    print(good_nodes(t1))
    assert good_nodes(t1) == result

test_good_nodes()
