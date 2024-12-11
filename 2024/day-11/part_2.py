import sys
from pathlib import Path
import timeit
from functools import lru_cache


@lru_cache(maxsize=None)
def processStone(stone: str, n):
    if n == 0:
        return 1
    if stone == '0':
        return processStone('1', n-1)
    if len(stone) % 2 == 0:
        l, r = str(int(stone[:len(stone)//2])), str(int(stone[len(stone)//2:]))
        return processStone(l, n-1) + processStone(r, n-1)
    return processStone(str(int(stone)*2024), n-1)


def main(input: str):
    stones = input.strip().split(' ')
    res = sum([processStone(stone, 75) for stone in stones])
    print(res)


if __name__ == '__main__':
    def readFile(filename):
        with open(filename, 'r') as f:
            return f.read()
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
