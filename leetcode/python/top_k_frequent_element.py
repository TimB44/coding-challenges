from typing import Counter, List
import random


class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        item_freq = Counter(nums)
        distinct_nums = list(map(lambda x: (item_freq[x], x), list(set(nums))))

        return list(map(lambda x: x[1], self.top_k(distinct_nums, k)))

    def top_k(self, nums: List[tuple[int, int]], k: int) -> List[tuple[int, int]]:
        pivot = self.get_pivot(nums)
        less_then = list(filter(lambda x: x[0] < nums[pivot][0], nums))
        greater_then = list(
            filter(lambda x: x[1] != nums[pivot][1] and x[0] >= nums[pivot][0], nums)
        )
        if len(greater_then) + 1 == k:
            greater_then.append(nums[pivot])
            return greater_then
        if len(greater_then) + 1 > k:
            return self.top_k(greater_then, k)
        num_left = k - 1 - len(greater_then)
        greater_then.append(nums[pivot])
        greater_then.extend(self.top_k(less_then, num_left))
        return greater_then

    def get_pivot(self, nums: List[tuple[int, int]]) -> int:
        return random.randrange(0, len(nums))
