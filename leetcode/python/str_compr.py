class Solution:
    def compressedString(self, word: str) -> str:
        out = []
        cur_count = 0
        cur_char = "A"
        for char in word:
            if char != cur_char or cur_count == 9:
                out.append(str(cur_count) + cur_char)
                cur_char = char
                cur_count = 0
            cur_count += 1

        out.append(str(cur_count) + cur_char)

        return "".join(out[1:])
