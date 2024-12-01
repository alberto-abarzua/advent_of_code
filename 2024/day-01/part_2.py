
import sys
from pathlib import Path
from collections import defaultdict


def main(input: str):
    l1, l2 = [], []
    numbers = [x.split('   ') for x in input.splitlines()]
    for a, b in numbers:
        l1.append((int(a)))
        l2.append(int(b))
    num_times_l2 = defaultdict(lambda: 0)

    for num in l2:
        num_times_l2[num] += 1
    total = [num*num_times_l2[num] for num in l1]
    print(sum(total))


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
