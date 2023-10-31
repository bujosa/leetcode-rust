from typing import List
from collections import deque

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def build_tree(pre_order: List[int], in_order: List[int]) -> TreeNode:
    def helper(pre_order: List[int] , in_order: List[int]):
        if not pre_order or not in_order:
            return None
        root = TreeNode(pre_order[0])
        mid = in_order.index(pre_order[0])
        root.left = helper(pre_order[1:mid+1], in_order[:mid])
        root.right = helper(pre_order[mid+1:], in_order[mid+1:])
        return root
        
    return helper(pre_order,in_order)

def test_build_tree():
    pre_order = [3,9,20,15,7]
    in_order = [9,3,15,20,7]
    t1 = TreeNode(3)
    t2 = TreeNode(9)
    t3 = TreeNode(20)
    t4 = TreeNode(15)
    t5 = TreeNode(7)

    t1.left = t2
    t1.right = t3
    t3.left = t4
    t3.right = t5

    assert build_tree(pre_order, in_order).right.val == 20

    print(build_tree(pre_order, in_order).right.val)


test_build_tree()
