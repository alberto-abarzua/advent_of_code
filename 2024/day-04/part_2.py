import sys
from typing import List
from pathlib import Path


def checkXMAS(lines: List[str], i: int, j: int):
    center = lines[i][j]
    opposite = {
        'M': 'S',
        'S': 'M',
    }
    if center != 'A':
        return False
    size = len(lines)

    if i == 0 or j == 0 or i > size-2 or j > size-2:
        return False

    top_left = lines[i-1][j-1]
    top_right = lines[i-1][j+1]
    bottom_left = lines[i+1][j-1]
    bottom_right = lines[i+1][j+1]
    vals = [top_left, top_right, bottom_left, bottom_right]
    if any([x not in opposite.keys() for x in vals]):
        return False

    if top_left == opposite[bottom_right] and top_right == opposite[bottom_left]:
        return True


def main(input: str):
    lines: List[str] = input.splitlines()
    print(input)
    total = 0
    assert len(lines) == len(lines[0])
    for i in range(len(lines)):
        for j in range(len(lines)):
            if checkXMAS(lines, i, j):
                total += 1

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
