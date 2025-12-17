from typing import List


# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children


class Solution:
    def postorder(self, root: "Node") -> List[int]:
        if root == None:
            return []
        l = []
        self.postOrderRec(root, l)
        return l

    def postOrderRec(self, cur, trav):
        for child in cur.children:
            self.postOrderRec(child, trav)

        trav.append(cur.val)
