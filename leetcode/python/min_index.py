class Solution:
    def minimumIndex(self, nums: list[int]) -> int:
        candidate = nums[0]
        candidate_count = 1
        for num in nums[1:]:
            candidate_count += 1 if num == candidate else -1
            if candidate_count == 0:
                candidate = num
                majority_count = 1

        # Now we know candidate holds the majority element
        majority_element = candidate
        majority_count = nums.count(majority_element)

        left_count = 0
        for i, n in enumerate(nums):
            left_count += 1 if majority_element == n else 0
            right_count = majority_count - left_count
            left_size = i + 1
            right_size = len(nums) - left_size
            if left_count > left_size // 2 and right_count > right_size // 2:
                return i

        return -1
