from typing import List
from collections import deque

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def level_order(root: 'TreeNode') -> List[List[int]]:
    result = []
    if not root:
        return result
    queue = deque([root])
    while queue:
        level = []
        next_queue = deque()
        for node in queue:
            level.append(node.val)
            if node.left:
                next_queue.append(node.left)
            if node.right:
                next_queue.append(node.right)
        if level:
            result.append(level)
        queue = next_queue
    return result

def test_level_order():
    t1 = TreeNode(3)
    t2 = TreeNode(9)
    t3 = TreeNode(20)
    t4 = TreeNode(15)
    t5 = TreeNode(7)
    t2.left = t4
    t2.right = t5
    t1.left = t2
    t1.right = t3
    result = [[3], [9, 20], [15, 7]]
    print(level_order(t1))
    assert level_order(t1) == result

test_level_order()