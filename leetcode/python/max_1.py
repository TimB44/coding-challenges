from typing import List

from itertools import chain 
class Solution:
    def longestSubarray(self, nums: List[int]) -> int:
        arr = [nums[0]]
        for num in nums[1:]:
            if num == 0:
                arr.append(0)
            elif arr[-1] != 0:
                arr[-1] = arr[-1] + 1
            else:
                arr.append(1)

        if len(arr) == 1:
            return max(arr[0] - 1, 0)

        return max(
            chain(( arr[i - 1] + arr[i + 1] for i, n in enumerate(arr) if n == 0 and i > 0 and i < len(arr) - 1), arr)
        )
