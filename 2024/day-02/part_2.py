import sys
from pathlib import Path


def is_safe(report):
    diff_sign = report[1] - report[0] > 0
    for i in range(1, len(report)):
        dif = report[i] - report[i-1]
        if ((dif > 0) != diff_sign) or (abs(dif) < 1 or abs(dif) > 3):
            return False
    return True


def main(input: str):
    reports = [x.split(' ') for x in input.splitlines()]
    reports = [list(map(int, l)) for l in reports]

    total = 0
    for report in reports:
        if is_safe(report):
            total += 1
            continue

        for i in range(len(report)):
            new_report = report[:i] + report[i+1:]
            if is_safe(new_report):
                total += 1
                break

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
