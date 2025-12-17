class Solution:
    def isCircularSentence(self, sentence: str) -> bool:
        if sentence[0] != sentence[-1]:
            return False
        return all(
            map(
                lambda x: sentence[x[0] - 1] == sentence[x[0] + 1],
                filter(lambda x: x == " ", enumerate(sentence)),
            )
        )
