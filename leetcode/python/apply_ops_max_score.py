from math import sqrt

MAX = 100000
MOD = 1000000007
MAX_FACTOR = int(sqrt(MAX))
FACTORS = [0] * (MAX_FACTOR + 1)


class Solution:
    def maximumScore(self, nums: list[int], k: int) -> int:
        prime_score = [numDistinctFactors(n) for n in nums]
        nums_sorted = sorted((n, i) for (i, n) in enumerate(nums))
        total_score = 1

        while k:
            n, i = nums_sorted.pop()
            n_score = prime_score[i]
            left_len = 0
            j = i - 1
            while j >= 0 and prime_score[j] < n_score:
                left_len += 1
                j -= 1

            right_len = 0
            j = i + 1
            while j < len(nums) and prime_score[j] <= n_score:
                right_len += 1
                j += 1
            total_moves = min(max((left_len + 1) * (right_len + 1), 1), k)
            k -= total_moves
            score = 1
            for _ in range(total_moves):
                score *= n
                score %= MOD
            total_score = (total_score * score) % MOD
        return total_score


def numDistinctFactors(n: int) -> int:
    if n == 1:
        return 0
    global FACTORS
    for i in range(len(FACTORS)):
        FACTORS[i] = 0
    breakDown(n)
    return sum(1 for factor in FACTORS if factor != 0)


def breakDown(n: int):
    for divsior in range(2, MAX_FACTOR + 1):
        if n == divsior:
            break

        if n % divsior == 0:
            breakDown(divsior)
            breakDown(n // divsior)
            return

    # Base case for prime numbers
    if n <= MAX_FACTOR:
        FACTORS[n] += 1
    else:
        # Case where the inital number is prime
        FACTORS[0] += 1
