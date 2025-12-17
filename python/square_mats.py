from typing import List


class Solution:
    def numSubmat(self, mat: List[List[int]]) -> int:
        m = len(mat)
        n = len(mat[0])

        up_streaks = [[0] * n for _ in range(m)]
        for col in reversed(range(n)):
            cur_streak = 0
            for row in reversed(range(m)):
                if mat[row][col] == 0:
                    cur_streak = 0
                else:
                    cur_streak += 1
                up_streaks[row][col] = cur_streak


        total = 0
        for row in range(m):
            for col in range(n):
                us = up_streaks[row][col]
                for height in range(us):
                    for length in range(n -col):
                        if up_streaks[row][col + length] <= height:
                            break
                        total += 1
        return total


def count_squares(n: int, m: int) -> int:
    sum = 0
    for a in range(1, n + 1):
        for b in range(1, m + 1):
            sum += (a * b)

    return sum
    



s = Solution()
s.numSubmat([
    [1, 0, 1],
    [1, 1, 0]
])
        
