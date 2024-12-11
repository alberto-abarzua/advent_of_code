import sys
from pathlib import Path
import timeit

def blink_iterative(stones):
    idx = 0
    while idx < len(stones):
        cur_stone = stones[idx]
        if cur_stone == '0':
            stones[idx] = '1'
            idx += 1
            continue

        if len(cur_stone) % 2 == 0 and cur_stone != '0':
            first_half, second_half = cur_stone[:len(cur_stone)//2], cur_stone[len(cur_stone)//2:]
            stones[idx] = str(int(first_half))
            stones.insert(idx+1, str(int(second_half)))
            idx += 2
            continue

        else:
            stones[idx] = str(int(cur_stone)*2024)
            idx += 1
    return stones


def main(input: str):
    stones = input.strip().split(' ')
    print(stones)
    for i in range(25):
        print(i,end='\r')
        blink_iterative(stones)
    print(stones)
    print(len(stones))


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
