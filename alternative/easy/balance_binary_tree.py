class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None

def is_balanced(root):
    def height(node):
        if not node:
            return 0
        left_height = height(node.left)
        right_height = height(node.right)
        if left_height == -1 or right_height == -1 or abs(left_height - right_height) > 1:
            return -1
        return max(left_height, right_height) + 1

    return height(root) != -1

# Test cases
t1 = TreeNode(3)
t2 = TreeNode(9)
t3 = TreeNode(20)
t4 = TreeNode(15)
t5 = TreeNode(7)
t2.left = t4
t2.right = t5
t1.left = t2
t1.right = t3
assert is_balanced(t1) == True

t1 = TreeNode(1)
t2 = TreeNode(2)
t3 = TreeNode(2)
t4 = TreeNode(3)
t5 = TreeNode(3)
t6 = TreeNode(4)
t7 = TreeNode(4)
t6.left = t7
t5.left = t6
t4.left = t5
t3.left = t4
t2.left = t3
t1.left = t2
assert is_balanced(t1) == False