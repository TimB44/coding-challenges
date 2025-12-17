from typing import Counter, List
import heapq
from sortedcontainers import SortedDict


class Solution:
    def maxPoints(self, grid: List[List[int]], queries: List[int]) -> List[int]:
        rows = len(grid)
        cols = len(grid[0])
        seen: set[tuple[int, int]] = set()
        q: list[tuple[int, int, int]] = [(grid[0][0], 0, 0)]
        cost: Counter[int] = Counter()

        while q:
            coords = heapq.heappop(q)
            cost, row, col = coords
            if (row, col) in seen:
                continue
            seen.add((row, col))
            squares_needing[cost] += 1

            for dx, dy in [[0, 1], [1, 0], [-1, 0], [0, -1]]:
                new_row = row + dy
                new_col = col + dx
                if (
                    new_row < 0
                    or new_col < 0
                    or new_row >= rows
                    or new_col >= cols
                    or (new_row, new_col) in seen
                ):
                    continue

                new_cost = max(cost, grid[new_row][new_col])
                heapq.heappush(q, (new_cost, new_row, new_col))

        cumulitive_cost = sorted(squares_needing.items())
        for i in range(1, len(cumulitive_cost)):
            cumulitive_cost[i] = (
                cumulitive_cost[i][0],
                cumulitive_cost[i][1] + cumulitive_cost[i - 1][1],
            )

        cur_cost = 0
        num_squares = 0
        results = []
        for query in queries:
            while (
                cur_cost < len(cumulitive_cost) and query > cumulitive_cost[cur_cost][0]
            ):
                num_squares = cumulitive_cost[cur_cost][1]
                cur_cost += 1
            results.append(num_squares)

        return results
