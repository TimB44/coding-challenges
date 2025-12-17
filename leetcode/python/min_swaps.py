class Solution:
    def minSwaps(self, s: str) -> int:
        balance = 0
        swaps = 0
        l, h = 0, len(s) - 1
        while l < h:
            if s[l] == "[":
                balance += 1
                continue
            if balance > 0:
                balance -= 1
                continue

            l += 1
            while s[h] == "]":
                h -= 1
            swaps += 1
        return swaps
