from typing import List, Tuple
import heapq


class Solution:
    def smallestRange(self, nums: List[List[int]]) -> List[int]:
        k = len(nums)
        heap = [(nums[i][0], i) for i in range(k)]
        max_num = max(map(lambda x: x[0], heap))
        heapq.heapify(heap)
        min_range = [heap[0][0], max_num]
        l = [0] * k
        while len(heap) == k:
            _, list_index = heapq.heappop(heap)
            l[list_index] += 1
            if l[list_index] == len(nums[list_index]):
                continue
            next_num = nums[list_index][l[list_index]]
            heapq.heappush(heap, (next_num, list_index))
            max_num = max(max_num, next_num)
            if max_num - heap[0][0] < min_range[1] - min_range[0]:
                min_range = [heap[0][0], max_num]

        return min_range
