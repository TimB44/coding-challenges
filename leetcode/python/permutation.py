from typing import Counter


class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        n = len(s2)
        target: Counter = Counter(s1)
        cur: Counter = Counter()
        l, h = 0, 0
        while h < n:
            cur_char = s2[h]
            cur[cur_char] += 1
            while cur[cur_char] > target[cur_char]:
                cur[s2[l]] -= 1
                l += 1
            if h - l + 1 == len(s1):
                return True
            h += 1

        return False
