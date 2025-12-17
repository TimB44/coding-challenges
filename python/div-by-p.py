from typing import List


class Solution:
    def minSubarray(self, nums: List[int], p: int) -> int:
        s = sum(nums)
        remove = s % p
        if remove == 0:
            return 0

        offset = 0
        shortestSumStart = {}
        shortestSubarray = len(nums)

        for i, num in enumerate(nums):
            num = num % p
            offset = (offset + (p - num)) % p
            shortestSumStart[(num + offset) % p] = i
            removeIdx = (remove + offset) % p
            if removeIdx in shortestSumStart:
                start = shortestSumStart[removeIdx]
                shortestSubarray = min(shortestSubarray, i - start + 1)

        if shortestSubarray == len(nums):
            return -1

        return shortestSubarray
