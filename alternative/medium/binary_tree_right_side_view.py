from typing import List
from collections import deque

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def right_side_view(root: 'TreeNode') -> List[List[int]]:
    result = []
    if not root:
        return result
    queue = deque([root])
    while queue:
        node = queue.pop()
        result.append(node.val)
        if node.right:
            queue.append(node.right)
        
    return result

def test_right_side_view():
    t1 = TreeNode(1)
    t2 = TreeNode(3)
    t1.right = t2
    result = [1, 3]
    print(right_side_view(t1))
    assert right_side_view(t1) == result

test_right_side_view()
