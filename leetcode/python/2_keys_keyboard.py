import math
from typing import Dict


class Solution:
    def minSteps(self, n: int) -> int:
        return self.minStepsRec(n, {})

    def minStepsRec(self, n: int, dp: Dict[int, int]) -> int:
        print(f"doing{n}")
        if n in dp:
            print(f"skipped {n}")
            return dp[n]
        print()
        if n == 1:
            return 0
        min_steps = n
        for i in range(2, math.floor(math.sqrt(n))):
            if n % i != 0:
                print(f"inner skipped n={n}, i={i}")
                continue
            min_steps = min(
                min_steps,
                self.minStepsRec(i, dp) + (n // i),
                self.minStepsRec((n // i), dp) + i,
            )
        dp[n] = min_steps
        return min_steps
