import sys
from pathlib import Path
import timeit
from typing import List
from dataclasses import dataclass
# Not the best but works (6s runtime)


@dataclass
class Slot:
    id: int | None
    size: int

    def __str__(self):
        return f"Slot({self.id}, {self.size})"

    def __repr__(self):
        return self.__str__()

    def __iter__(self):
        return iter([self.id, self.size])


def find_open(arr: List[Slot], right: int):
    r_slot = arr[right]
    for i in range(right):
        if arr[i].id is None and arr[i].size >= r_slot.size:
            return i


def mergeNones(arr: List[Slot]):
    # merges the sizes of consecutive None slots
    start = 0
    arr = arr.copy()
    while start < len(arr):

        if arr[start].id is not None:
            start += 1
            continue

        end = start
        while end < len(arr) and arr[end].id is None:
            end += 1

        total_size = sum([x.size for x in arr[start:end]])
        # delete the None slots
        for _ in range(start, end):
            arr.pop(start)

        arr.insert(start, Slot(None, total_size))
        start += 1
    return arr


def main(input: str):
    nums = [int(x) for x in input.strip()]

    arr: List[Slot] = []
    for i, num in enumerate(nums):

        if i % 2 == 0:
            arr.append(Slot(i//2, num))
        else:
            arr.append(Slot(None, num))

    right = len(arr)-1
    while right >= 0:
        print(right, end="\r")
        if arr[right].id is None:
            right -= 1
            continue

        left = find_open(arr, right)
        if left is None:
            right -= 1
            continue

        dif = arr[left].size - arr[right].size
        arr[left] = arr[right]
        arr[right] = Slot(None, arr[left].size)
        if dif > 0:
            arr.insert(left+1, Slot(None, dif))
            arr = mergeNones(arr)
        right -= 1

    total = 0
    id = 0
    for slot in arr:
        for _ in range(slot.size):
            if slot.id is None:
                id += 1
                continue
            total += id * slot.id
            id += 1

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
