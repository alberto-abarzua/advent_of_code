import sys
from pathlib import Path
import timeit
from collections import defaultdict
from math import ceil


def are_collinear(p1, p2, p3):
    x1, y1 = p1
    x2, y2 = p2
    x3, y3 = p3
    return (y2-y1)*(x3-x1) == (y3-y1)*(x2-x1)

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
        for i, p1 in enumerate(value):
            for j, p2 in enumerate(value):
                if i != j:
                    # Check every point in the grid for collinearity
                    for x in range(size):
                        for y in range(size):
                            if are_collinear(p1, p2, (x, y)):
                                valid_positions.add((x, y))
                                
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
