from typing import List
import heapq


class Solution:
    def smallestChair(self, times: List[List[int]], targetFriend: int) -> int:
        avaliable_chairs = [i for i in range(len(times))]
        used_chair = []

        for person, duration in sorted(enumerate(times), key=lambda x: x[1]):
            start, end = duration
            while len(used_chair) > 0 and used_chair[0][0] <= start:
                heapq.heappush(avaliable_chairs, heapq.heappop(used_chair)[1])
            chair = heapq.heappop(avaliable_chairs)
            if person == targetFriend:
                return chair
            heapq.heappush(used_chair, (end, chair))

        # Should never reach
        return -1
