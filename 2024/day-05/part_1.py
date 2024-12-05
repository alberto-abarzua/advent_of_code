import sys
from pathlib import Path
from collections import defaultdict
from typing import Dict, Union


def check_update(rules, update):
    indices: Dict[int, Union[int, None]] = defaultdict(lambda: None)
    for i, num in enumerate(update):
        indices[num] = i
    for (a, b) in rules:
        a_pos = indices[a]
        b_pos = indices[b]
        if a_pos == None or b_pos == None:
            continue

        if a_pos > b_pos:
            return False

    return True

def main(input: str):
    rules, updates = input.split('\n\n')
    rules = [list(map(int, x.split('|'))) for x in rules.splitlines()]
    updates = [list(map(int, x.split(","))) for x in updates.splitlines()]
    total = 0
    for update in updates:
        if check_update(rules, update):
            total += update[len(update) // 2]
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
