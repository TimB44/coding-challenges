from typing import List
import heapq
import math


class Solution:
    def maxKelements(self, nums: List[int], k: int) -> int:
        heap = list(map(lambda x: -x, nums))
        heapq.heapify(heap)

        score = 0
        for i in range(k):
            item = heapq.heappop(heap)
            item = -item
            score += item
            heapq.heappush(heap, -math.ceil(item / 3))

        return score
