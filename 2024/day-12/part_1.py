from collections import defaultdict
import sys
from pathlib import Path
import timeit
from typing import List, Tuple, Dict, TypeAlias, Set

ZonesType: TypeAlias = Dict[Tuple[str, int], Set[Tuple[int, int]]]

directions = {
    'up': (-1, 0),
    'down': (1, 0),
    'left': (0, -1),
    'right': (0, 1)
}


def fill(grid: List[List[str]], pos: Tuple[int, int], init_info: Tuple[str, int], zones: ZonesType):
    x, y = pos
    initial, id = init_info

    if x not in range(len(grid)) or y not in range(len(grid[0])):
        return

    if grid[x][y] != initial or (x, y) in zones[(initial, id)]:
        return

    zones[(initial, id)].add((x, y))
    for dx, dy in directions.values():
        fill(grid, (x + dx, y + dy), init_info, zones)


def main(input: str):
    grid: List[List[str]] = [list(row)for row in input.splitlines()]
    zones: ZonesType = defaultdict(set)
    id = 0

    points = set((x, y) for x in range(len(grid)) for y in range(len(grid[0])))

    while points:
        pos = next(iter(points))  # Get an arbitrary element from the set
        x, y = pos

        fill(grid, pos, (grid[x][y], id), zones)
        points = points - zones[(grid[x][y], id)]  # Modify the points set
        id += 1

    num_neighbors: Dict[Tuple[int, int], int] = defaultdict(lambda: 0)
    for zone in zones.values():
        for x, y in zone:
            for dx, dy in directions.values():
                if (x+dx) in range(len(grid)) and (y+dy) in range(len(grid[0])):
                    cur = grid[x][y]
                    neighbor = grid[x + dx][y + dy]
                    if cur != neighbor:
                        num_neighbors[(x, y)] += 1
                else:
                    num_neighbors[(x, y)] += 1

    total = 0
    for zone in zones.values():
        cur_total = 0

        for x, y in zone:
            cur_total += num_neighbors[(x, y)]

        total += cur_total*len(zone)

    print(total)


if __name__ == '__main__':
    runSample = False
    CWD = Path(__file__).parent

    def readFile(filename):
        with open(filename, 'r') as f:
            return f.read()

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
