from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def kthLargestLevelSum(self, root: Optional[TreeNode], k: int) -> int:
        if root == None:
            return -1
        sums = [0]

        def dfs(cur: Optional[TreeNode], level: int):
            if cur == None:
                return
            if level == len(sums):
                sums.append(0)
            sums[level] += cur.val

            dfs(cur.left, level + 1)
            dfs(cur.right, level + 1)

        dfs(root, 0)
        sums.sort(reverse=True)
        return sums[k - 1]
