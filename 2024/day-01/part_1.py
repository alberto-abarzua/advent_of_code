import sys
from pathlib import Path


def main(input: str):
    print("Running main function", input)
    l1, l2 = [],[]
    numbers = [x.split('   ') for x in input.splitlines()]
    for a, b in numbers:
        l1.append((int(a)))
        l2.append(int(b))

    l1 = sorted(l1)
    l2 = sorted(l2)
    dif = sum([abs(a - b) for a, b in zip(l1, l2)])
    print(dif)


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
