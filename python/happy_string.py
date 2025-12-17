class Solution:
    def longestDiverseString(self, a: int, b: int, c: int) -> str:
        out = []
        while a + b + c:
            if a > b and a > c:
                if cannot_append(out, "a"):
                    if b == 0 and c == 0:
                        break
                    if b > c:
                        b -= 1
                        out.append("b")
                    else:
                        c -= 1
                        out.append("c")
                else:
                    a -= 1
                    out.append("a")
            elif b > c:
                if cannot_append(out, "b"):
                    if c == 0 and a == 0:
                        break
                    if c > a:
                        out.append("c")
                        c -= 1
                    else:
                        out.append("a")
                        a -= 1
                else:
                    out.append("b")
                    b -= 1
            else:
                if cannot_append(out, "c"):
                    if a == 0 and b == 0:
                        break
                    if a > b:
                        a -= 1
                        out.append("a")
                    else:
                        b -= 1
                        out.append("b")
                else:
                    c -= 1
                    out.append("c")
        return "".join(out)


def cannot_append(out, char):
    return len(out) > 1 and out[-1] == char and out[-2] == char
