from typing import Any, Optional, List

from itertools import chain


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def treeQueries(self, root: Optional[TreeNode], queries: List[int]) -> List[int]:
        if root == None:
            return []

        n = dfs1(root)
        subtree_height = [-1] * (n + 1)

        def dfs2(cur) -> int:
            if cur == None:
                return -1
            subtree_height[cur.val] = max(dfs2(cur.left), dfs2(cur.right)) + 1
            return subtree_height[cur.val]

        answers = [-1] * (n + 1)
        cur_level_depth = 1
        cur_level = []
        if root.left != None:
            cur_level.append(root.left)

        if root.right != None:
            cur_level.append(root.right)

        next_level = []

        while cur_level:
            max1 = max(cur_level, key=lambda x: subtree_height[x.val])
            max2 = max(
                chain(filter(lambda x: x != max1, cur_level), [TreeNode(0)]),
                key=lambda x: subtree_height[x.val],
            )
            print(max1.val)
            print(max2.val)
            for node in cur_level:
                if node == max1:
                    answers[node.val] = cur_level_depth + subtree_height[max2.val]
                else:
                    answers[node.val] = cur_level_depth + subtree_height[max1.val]
                if node.left != None:
                    next_level.append(node.left)
                if node.right != None:
                    next_level.append(node.right)

            cur_level_depth += 1
            cur_level = next_level
            next_level = []

        return list(map(lambda x: answers[x], queries))


def dfs1(node: Optional[TreeNode]) -> int:
    if node == None:
        return 0
    return 1 + dfs1(node.left) + dfs1(node.right)
