class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None

def has_path_sum(root, sum):
    if root is None:
        return False

    stack = [(root, sum)]

    while stack:
        node, curr_sum = stack.pop()
        if node.left is None and node.right is None and node.val == curr_sum:
            return True
        if node.left is not None:
            stack.append((node.left, curr_sum - node.val))
        if node.right is not None:
            stack.append((node.right, curr_sum - node.val))
    return False

# Test case
t1 = TreeNode(5)
t2 = TreeNode(4)
t3 = TreeNode(8)
t4 = TreeNode(11)
t6 = TreeNode(4)
t7 = TreeNode(7)
t8 = TreeNode(2)
t9 = TreeNode(1)
t6.right = t9
t4.right = t8
t4.left = t7
t3.right = t6
t2.left = t4
t1.right = t3
t1.left = t2
assert has_path_sum(t1, 22) == True