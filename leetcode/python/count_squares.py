from typing import List


class Solution:
    def countSquares(self, matrix: List[List[int]]) -> int:
        rows = len(matrix)
        cols = len(matrix[0])

        def canExpand(startr: int, startc: int, n: int) -> bool:
            r = startr + n
            c = startc + n
            if r >= rows or c >= cols:
                return False
            for delta in range(n + 1):
                if matrix[r][startc + delta] != 1 or matrix[startr + delta][c] != 1:
                    return False
            return True

        ret = 0
        for row in range(rows):
            for col in range(cols):
                if matrix[row][col] == 1:
                    n = 1
                    while canExpand(row, col, n):
                        n += 1
                    ret += n

        return ret
