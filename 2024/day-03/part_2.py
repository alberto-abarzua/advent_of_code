import sys
import re
from pathlib import Path


def extractMuls(string: str):
    return re.findall(r'mul\((\d+),(\d+)\)', string)


def replaceBlocks(string: str):
    return re.sub(r"don't\(\).*?(do\(\)|$)", "", string, flags=re.DOTALL)


def main(input: str):

    new_input = replaceBlocks(input)
    print(new_input)
    muls = extractMuls(new_input)
    print(muls)
    res = sum([int(a) * int(b) for a, b in muls])
    print(res)


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
