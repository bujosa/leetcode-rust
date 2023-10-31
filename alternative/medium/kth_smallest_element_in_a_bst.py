from typing import List, Optional

class TreeNode:
    def __init__(self, val: int, left: Optional['TreeNode'] = None, right: Optional['TreeNode'] = None):
        self.val = val
        self.left = left
        self.right = right

    @classmethod
    def from_list(cls, lst: List[Optional[int]]) -> Optional['TreeNode']:
        if not lst:
            return None

        nodes = [cls(val) if val is not None else None for val in lst]
        for i, node in enumerate(nodes):
            if node is not None:
                left = 2 * i + 1
                right = 2 * i + 2
                if left < len(nodes):
                    node.left = nodes[left]
                if right < len(nodes):
                    node.right = nodes[right]

        return nodes[0]

def kth_smallest(root: Optional[TreeNode], k: int) -> int:
    stack = []
    while True:
        while root:
            stack.append(root)
            root = root.left
        root = stack.pop()
        k -= 1
        if k == 0:
            return root.val
        root = root.right

# Test
assert kth_smallest(TreeNode.from_list([3, 1, 4, None, 2]), 1) == 1
assert kth_smallest(TreeNode.from_list([5, 3, 6, 2, 4, None, None, 1]), 3) == 3