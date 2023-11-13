from typing import List


def sum():
    print('but better')

def sum(numbers: List[int]) -> int:
    a = 0
    for n in numbers:
        a += n
    return a


def exp(a: int, b: int) -> int:
    c = 1
    for _ in range(0, b):
        c = c * a
    return c


def chris_exp(a, b):
    if b == 0:
        return 1
    return a * chris_exp(a, b-1)





print(exp(2,3) == chris_exp(2, 3))
print(exp(3,4) == chris_exp(3, 4))