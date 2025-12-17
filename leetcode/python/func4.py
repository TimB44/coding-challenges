def func4(givenNum, sl, sh):
    print(givenNum, sl, sh)
    retval = sh - sl
    tmp = retval + 1 if retval < 0 else retval
    tmp //= 2
    tmp += sl
    print(tmp)
    if tmp > givenNum:
        return 2 * func4(givenNum, sl, tmp - 1)
    elif tmp < givenNum:
        return 2 * func4(givenNum, tmp + 1, sh) + 1
    return retval


# i = 14
# while True:
#     print(f"Trying {i}")
#     retval = func4(i, 0, 14)
#     print(retval)
#     i -= 1

ret = func4(8, 0, 14)
print()
print(ret)
