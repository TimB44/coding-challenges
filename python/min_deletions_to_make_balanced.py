class Solution:
    def minimumDeletions(self, s: str) -> int:
        a_count = s.count("a")
        a_seen = 0
        b_seen = 0
        min_cost = int("1e9")
        for i, c in enumerate(s):
            min_cost = min(min_cost, b_seen + (a_count - a_seen))
            if c == "a":
                a_seen += 1
            else:
                b_seen += 1
        return min_cost
