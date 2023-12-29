import math, re

board = list(open("./inputs/day3-part1.txt"))
chars = {
    (r, c): []
    for r in range(140)
    for c in range(140)
    if board[r][c] not in "01234566789."
}

for row_index, row in enumerate(board):
    for number in re.finditer(r"\d+", row):
        edge = {
            (r, c)
            for r in (row_index - 1, row_index, row_index + 1)
            for c in range(number.start() - 1, number.end() + 1)
        }

        for o in edge & chars.keys():
            chars[o].append(int(number.group()))

print(
    sum(sum(p) for p in chars.values()),
    sum(math.prod(p) for p in chars.values() if len(p) == 2),
)
