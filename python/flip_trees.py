from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def flipEquiv(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> bool:
        treeSort(root1)
        treeSort(root2)
        return treeEquals(root1, root2)


def treeSort(cur: Optional[TreeNode]):
    if cur == None:
        return
    if cur.left == None:
        left_val = -1
    else:
        left_val = cur.left.val

    if cur.right == None:
        right_val = -1
    else:
        right_val = cur.right.val

    if right_val < left_val:
        cur.left, cur.right = cur.right, cur.left

    treeSort(cur.left)
    treeSort(cur.right)


def treeEquals(cur1: Optional[TreeNode], cur2: Optional[TreeNode]) -> bool:
    if cur1 == None and cur2 == None:
        return True
    if cur1 == None or cur2 == None or cur1.val != cur2.val:
        return False
    return treeEquals(cur1.left, cur2.left) and treeEquals(cur1.right, cur2.right)
