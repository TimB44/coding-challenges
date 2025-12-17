from typing import List
import heapq


class Solution:
    def modifiedGraphEdges(
        self, n: int, edges: List[List[int]], source: int, destination: int, target: int
    ) -> List[List[int]]:
        return []

    def maxProbability(
        self,
        n: int,
        edges: List[List[int]],
        succProb: List[float],
        start_node: int,
        end_node: int,
    ) -> float:
        adj = [[] for _ in range(n)]
        for i in range(len(edges)):
            v1, v2 = edges[i]
            cur_path_prob = succProb[i]

            adj[v1].append((v2, cur_path_prob))
            adj[v2].append((v1, cur_path_prob))

        max_prob = [0] * n
        max_prob[start_node] = 1
        seen = [False] * n
        # Store probablilites as negative in order for min heap to work
        q = [(-1, start_node)]

        while q:
            cur_path_prob, cur_node = heapq.heappop(q)
            cur_path_prob = -cur_path_prob
            seen[cur_node] = True

            for dest, p in adj[cur_node]:
                if seen[dest]:
                    continue
                if p * cur_path_prob > max_prob[dest]:
                    max_prob[dest] = p * cur_path_prob
                    heapq.heappush(q, (-max_prob[dest], dest))

        return max_prob[end_node]
