import heapq


class Solution:
    def nthUglyNumber(self, n: int) -> int:
        seen = set()
        heap = [1]
        for _ in range(n - 1):
            smallest = heapq.heappop(heap)
            for factor in [2, 3, 5]:
                new_num = smallest * factor
                if new_num not in seen:
                    seen.add(new_num)
                    heapq.heappush(heap, new_num)

        return heap[0]
