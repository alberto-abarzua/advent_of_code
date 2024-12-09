import sys
from pathlib import Path
import timeit
from collections import defaultdict


def line_equation(p1, p2):
    x1, y1 = p1
    x2, y2 = p2
    m = (y2 - y1) / (x2 - x1)
    c = y1 - m * x1
    return lambda x: m * x + c

def euclidean_distance(p1, p2):
    x1, y1 = p1
    x2, y2 = p2
    return ((x1 - x2) ** 2 + (y1 - y2) ** 2) ** 0.5

def main(input: str):
    grid = input.splitlines()
    positions = defaultdict(lambda: [])

    for x, row in enumerate(grid):
        for y, cell in enumerate(row):
            if cell != '.':
                positions[cell].append((x, y))
    valid_positions = set()
    size = len(grid)
    for value in positions.values():
        for val in value:
            for val2 in value:
                if val != val2:
                    for x in range(size):
                        y = line_equation(val, val2)(x)
                        # valid y is inte or .5
                        if (0 <= y and y < size):
                            y = round(y)
                            d_val = euclidean_distance(val, (x, y))
                            d_val2 = euclidean_distance(val2, (x, y))
                            if d_val == 2*d_val2 or d_val2 == 2*d_val:
                                valid_positions.add((x, y))
                        else:
                            print(f"y is not integer: {y}")

    print(len(valid_positions))



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
