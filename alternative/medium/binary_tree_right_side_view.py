from typing import List
from collections import deque

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def right_side_view(root: 'TreeNode') -> List[List[int]]:
    res = []
    q = deque([root])

    while q:
        rightSide = None
        qLen = len(q)

        for i in range(qLen):
            node = q.popleft()
            if node:
                rightSide = node
                q.append(node.left)
                q.append(node.right)
        if rightSide:
            res.append(rightSide.val)
    return res

def test_right_side_view():
    t1 = TreeNode(1)
    t2 = TreeNode(3)
    t1.right = t2
    result = [1, 3]
    print(right_side_view(t1))
    assert right_side_view(t1) == result

test_right_side_view()
