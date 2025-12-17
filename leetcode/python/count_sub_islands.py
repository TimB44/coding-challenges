from typing import Set, Tuple, List


class Solution:
    def countSubIslands(self, grid1: List[List[int]], grid2: List[List[int]]) -> int:
        rows = len(grid1)
        cols = len(grid1[0])
        seen1 = [[False] * cols for _ in range(rows)]
        seen2 = [[False] * cols for _ in range(rows)]
        count = 0
        for row in range(rows):
            for col in range(cols):
                if (
                    seen2[row][col] != False
                    or grid1[row][col] == 0
                    or grid2[row][col] == 0
                ):
                    continue

                s1 = set()
                if seen1[row][col] == False:
                    self.dfs(grid1, seen1, row, col, s1)
                else:
                    s1 = seen1[row][col]

                s2 = set()
                self.dfs(grid2, seen2, row, col, s2)
                if s2.issubset(s1):
                    count += 1
        return count

    def dfs(
        self,
        grid: List[List[int]],
        seen,
        row: int,
        col: int,
        s: Set[Tuple[int, int]],
    ):
        rows = len(grid)
        cols = len(grid[0])
        if (
            row < 0
            or col < 0
            or row >= rows
            or col >= cols
            or grid[row][col] == 0
            or seen[row][col]
        ):
            return
        seen[row][col] = s
        s.add((row, col))
        self.dfs(grid, seen, row + 1, col, s)
        self.dfs(grid, seen, row - 1, col, s)
        self.dfs(grid, seen, row, col + 1, s)
        self.dfs(grid, seen, row, col - 1, s)
