import random
from typing import List


def solve(puzzle: List[List[chr]], want: str) -> int:
    n = len(want)
    if n == 0:
        return 0
    width = len(puzzle[0])
    height = len(puzzle)
    count = 0
    alan = list(want)
    for (i, row) in enumerate(puzzle):
        check_north = i - n > 0
        check_south = i + n <= height
        for (j, column) in enumerate(row):
            if column != want[0]:
                continue
            check_east = j - n > 0
            check_west = j + n <= width
            if check_east:
                candidate = row[j-n:j]
                candidate.reverse()
                if candidate == alan:
                    count += 1
            if check_west:
                candidate = row[j:j+n]
                if candidate == alan:
                    count += 1
            if check_north:
                candidate = [puzzle[r][j] for r in range(i, i - n, -1)]
                if candidate == alan:
                    count += 1
            if check_south:
                candidate = [puzzle[r][j] for r in range(i, i + n)]
                if candidate == alan:
                    count += 1
    return count

def produce(height: int, width: int) -> List[List[chr]]:
    options = ['A', 'L', 'N']
    return [[options[random.randrange(0, len(options))] for _ in range(0, width)] for _ in range(height)]

# got = solve(puzzle=[
#     ['a', 'l', 'a', 'n', 'a'],
#     ['a', 'l', 'l', 'n', 'l'],
#     ['a', 'l', 'a', 'n', 'a'],
#     ['a', 'l', 'n', 'n', 'n'],
#     ['a', 'l', 'a', 'n', 'p'],
# ], want='alan')
#
# print(got)
#
# input = produce(5000, 5000)
#
# import json
# with open("ALAN.json", "w+") as f:
#     json.dump(input, f)

import json
with open("/home/chris/projects/intro/alan/ALAN.json", "r") as f:
    input = json.load(f)

from time import time
start = time()
print(solve(input, "ALAN"))
print(time() - start)
