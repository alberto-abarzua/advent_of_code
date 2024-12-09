import sys
from pathlib import Path
import timeit
from typing import List

def main(input: str):
    print("Running main function", input)
    nums = [int(x) for x in input.strip()]
    arr: List[int | None] = []
    for i, num in enumerate(nums):

        if i % 2 == 0:
            new_arr = [i//2 for _ in range(num)]
            arr.extend(new_arr)
        else:
            new_arr = [None for _ in range(num)]
            arr.extend(new_arr)
    print(arr)

    left, right = 0, len(arr)-1
    while left < right:
        if arr[right] is None:
            right -= 1
            continue

        if arr[left] is not None:
            left += 1
            continue

        arr[left], arr[right] = arr[right], arr[left]
        left += 1
        right -= 1
    total = 0
    for i, num in enumerate(arr):
        if num is None:
            continue
        total += num * i
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
