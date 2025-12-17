from typing import List


class Solution:
    def longestMonotonicSubarray(self, nums: List[int]) -> int:
        last = -1
        # True is increasing, false is decreasing, None is equal
        dir: bool | None = None
        m = 1
        for i in range(1, len(nums)):
            if nums[i] == nums[i - 1]:
                new_dir = None
            elif nums[i] <= nums[i - 1]:
                new_dir = False
            else:
                new_dir = True

            if new_dir == None:
                last = i - 1
            elif new_dir != dir:
                last = i - 2
            dir = new_dir
            m = max(m, i - last)
        return m
