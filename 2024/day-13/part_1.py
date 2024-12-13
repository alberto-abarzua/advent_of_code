import sys
from pathlib import Path
import timeit
from dataclasses import dataclass
from typing import Tuple
import re


@dataclass
class Machine:
    button_a: Tuple[int, int]
    button_b: Tuple[int, int]
    prize: Tuple[int, int]

    def __str__(self):
        return f"Machine A: {self.button_a}  B: {self.button_b} P: {self.prize})"

    def __repr__(self):
        return self.__str__()


def main(input: str):
    cases = [[x for x in chunk.splitlines()] for chunk in input.split('\n\n')]
    machines = []
    for (button_a_info, button_b_info, prize_info) in cases:
        button_a = tuple(map(int, re.findall(r'\d+', button_a_info)))
        button_b = tuple(map(int, re.findall(r'\d+', button_b_info)))
        prize = tuple(map(int, re.findall(r'\d+', prize_info)))
        machines.append(Machine(button_a, button_b, prize))

    total = 0
    for machine in machines:
        ax, ay = machine.button_a
        bx, by = machine.button_b
        px, py = machine.prize
        b = (ax*py-ay*px)/(ax*by-ay*bx)
        a = (by*px-bx*py)/(ax*by-ay*bx)
        if not (a.is_integer() and b.is_integer()):
            continue
        total += 3*a+b if (a <= 100 and b <= 100) else 0

    print(int(total))


if __name__ == '__main__':
    runSample = False
    CWD = Path(__file__).parent

    if (len(sys.argv) >= 2):
        runSample = sys.argv[1]

    def readFile(filename):
        with open(filename, 'r') as f:
            return f.read()
    # files are ./sample.txt and ./input.txt
    sample = readFile(CWD / 'sample.txt')
    input = readFile(CWD / 'input.txt')

    if runSample:
        print("Running sample")
        start_time = timeit.default_timer()
        result = main(sample)
        end_time = timeit.default_timer()
        print(f"Sample execution time: {end_time - start_time:.4f} seconds")
    else:
        print("Running input")
        # Time the execution
        start_time = timeit.default_timer()
        result = main(input)
        end_time = timeit.default_timer()
        print(f"Input execution time: {end_time - start_time:.4f} seconds")
