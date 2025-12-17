class Solution:
    def longestSquareStreak(self, nums: List[int]) -> int:
        longest = {}
        for num in nums:
            longest[num] = -1

        def find_longest(num: int):



