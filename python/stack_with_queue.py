from collections import deque


class MyStack:

    def __init__(self):
        self.q = deque()

    def push(self, x: int) -> None:
        self.q.appendleft(x)

    def pop(self) -> int:
        for _ in range(len(self.q) - 1):
            self.q.appendleft(self.q.pop())

        return self.q.pop()

    def top(self) -> int:
        item = self.pop()
        self.push(item)
        return item

    def empty(self) -> bool:
        return len(self.q) == 0
