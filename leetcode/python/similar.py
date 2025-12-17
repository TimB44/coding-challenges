class Solution:
    def areSentencesSimilar(self, sentence1: str, sentence2: str) -> bool:
        if len(sentence1) == len(sentence2):
            return sentence1 == sentence2
        if len(sentence1) < len(sentence2):
            sentence1, sentence2 = sentence2, sentence1

        target_words = sentence1.split(" ")
        given_words = sentence2.split(" ")
        l = 0
        while l < len(given_words) and target_words[l] == given_words[l]:
            l += 1
        h = 0
        while (
            h < len(given_words)
            and target_words[len(target_words) - 1 - h]
            == given_words[len(given_words) - 1 - h]
        ):
            h += 1
        return l + h <= len(given_words)
