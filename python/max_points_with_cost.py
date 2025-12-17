from typing import List


class Solution:
    def maxPoints(self, points: List[List[int]]) -> int:
        row_len = len(points[0])
        dp = points[0].copy()
        for i in range(1, len(points)):
            max_prev = [0] * row_len
            cur_max_l = 0
            cur_max_r = 0
            for j in range(row_len):
                cur_max_l = max(cur_max_l - 1, dp[j])
                cur_max_r = max(cur_max_r - 1, dp[row_len - 1 - j])
                max_prev[i] = max(cur_max_l, max_prev[j])
                max_prev[row_len - 1 - j] = max(cur_max_r, max_prev[row_len - 1 - j])
            for j in range(row_len):
                dp[j] = max_prev[j] + points[i][j]

        return max(dp)
