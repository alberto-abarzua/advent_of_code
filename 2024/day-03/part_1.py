import sys
import re
from pathlib import Path


def extractMuls(string: str):
    # searchres the string for substrings in this form 'mul(NUMBER,NUMBER)'
    # returns a list of tuples with the two numbers
    return re.findall(r'mul\((\d+),(\d+)\)', string)


def main(input: str):
    muls = extractMuls(input)
    print(muls)
    res =sum([int(a) * int(b) for a, b in muls])
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
