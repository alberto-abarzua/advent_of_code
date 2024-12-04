import sys
import re
from typing import List
from pathlib import Path

patterns = ['XMAS', 'SAMX']
length = 4


def get_diagonals_left_to_right(lines: List[str]):
    diags = []
    for i in range(len(lines)):
        diags.append(''.join([lines[i + j][j] for j in range(len(lines) - i)]))

    for i in range(1, len(lines)):
        diags.append(''.join([lines[j][i + j] for j in range(len(lines) - i)]))
    return diags


def get_diagonals_right_to_left(lines: List[str]):
    diags = []
    n = len(lines)
    for i in range(n):
        diags.append(''.join([lines[i - j][j] for j in range(i + 1) if i - j >= 0]))

    for i in range(1, n):
        diags.append(''.join([lines[n - j - 1][i + j] for j in range(n - i) if i + j < n]))
    return diags


def main(input: str):
    lines: List[str] = input.splitlines()
    print(input)
    assert len(lines) == len(lines[0])
    diagonals = get_diagonals_left_to_right(lines) + get_diagonals_right_to_left(lines)
    for diag in diagonals:
        print(diag)
    cols = [''.join([row[i] for row in lines]) for i in range(len(lines))]

    total = 0
    for pattern in patterns:
        for line in lines:
            # total += line.count(pattern)
            total += len(re.findall(pattern, line))

        for diag in diagonals:
            # total += diag.count(pattern)
            total += len(re.findall(pattern, diag))
        for col in cols:
            # total += col.count(pattern)
            total += len(re.findall(pattern, col))
    print(total)


def readFile(filename):
    with open(filename, 'r') as f:
        return f.read()


if __name__ == '__main__':
    runSample = False
    CWD = Path(__file__).parent
    if (len(sys.argv) >= 2):
        runSample = sys.argv[1]

    # files are ./sample.txt and ./input.txt
    sample = readFile(CWD / 'sample.txt')
    input = readFile(CWD / 'input.txt')
    if runSample:
        print("Running sample")
        main(sample)
    else:
        print("Running input")
        main(input)
