from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def replaceValueInTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        sum_at_depth = [0]

        def dfs(cur, depth):
            if cur == None:
                return
            if len(sum_at_depth) == depth:
                sum_at_depth.append(0)

            sum_at_depth[depth] += cur.val
            dfs(cur.left, depth + 1)
            dfs(cur.right, depth + 1)

        def dfs2(cur, depth):
            if cur == None:
                return 0
            sum_of_children = dfs(cur.left, depth + 1)
            sum_of_children = dfs(cur.right, depth + 1)

            if sum_of_children == 0:
                return cur.val
            cousin_sums = sum_at_depth[depth + 1] - sum_of_children
            if cur.left != None:
                cur.left.val = cousin_sums
            if cur.right != None:
                cur.right.val = cousin_sums

            return sum_of_children + cur.val

        dfs(root, 0)
        dfs2(root, 0)
        return root
