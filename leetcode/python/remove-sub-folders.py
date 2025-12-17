from typing import Any, Dict, List


class Solution:
    def removeSubfolders(self, folder: List[str]) -> List[str]:
        t = Trie()
        for path in folder:
            t.addFile(path.split("/")[1:], 0)

        return t.getTopLevelFolders()


class Trie:
    def __init__(self):
        self.folders = {}
        self.end = False

    def addFile(self, path: List[str], i: int):
        if len(path) == i:
            self.end = True
            return
        if path[i] not in self.folders:
            self.folders[path[i]] = Trie()
        self.folders[path[i]].addFile(path, i + 1)

    def getTopLevelFolders(self) -> List[str]:
        if self.end:
            return [""]
        out = []
        for key, trie in self.folders.items():
            out.extend(map(lambda x: f"/{key}" + x, trie.getTopLevelFolders()))
        return out


Solution().removeSubfolders(["/a", "/a/b"])
