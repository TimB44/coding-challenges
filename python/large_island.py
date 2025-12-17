from typing import List


class Solution:
    def largestIsland(self, grid: List[List[int]]) -> int:
        n = len(grid)
        par = [
            [(-1, 1) if grid[r][c] == 1 else None for c in range(n)] for r in range(n)
        ]
        seen = [[False for _ in range(n)] for _ in range(n)]

        def dfs(row: int, col: int):
            if row < 0 or row >= n or col < 0 or col >= n:
                return
