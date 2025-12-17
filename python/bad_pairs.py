from collections import Counter
from typing import List


class Solution:
    def countBadPairs(self, nums: List[int]) -> int:
        n = len(nums)
        diffs = Counter()
        for i, num in enumerate(nums):
            diffs[num - i] += 1

        total_bad = 0
        for num_good in diffs.values():
            num_bad = n - num_good
            total_bad += (num_bad * num_bad) // 2

        return total_bad // 2
