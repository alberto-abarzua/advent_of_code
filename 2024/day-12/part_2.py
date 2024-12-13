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


def count_sides(grid: List[List[str]], zone: Set[Tuple[int, int]], zones: ZonesType) -> int:
    def check_cell(x: int, y: int) -> bool:
        if x < 0 or y < 0 or x >= len(grid) or y >= len(grid[0]):
            return False
        return (x, y) in zone

    corners = 0
    xs = [x for x, y in zone]
    ys = [y for x, y in zone]
    min_x, max_x = min(xs), max(xs)
    min_y, max_y = min(ys), max(ys)

    for x in range(min_x - 1, max_x + 2):
        for y in range(min_y - 1, max_y + 2):
            cells = [
                check_cell(x, y),      # top-left
                check_cell(x, y+1),    # top-right
                check_cell(x+1, y),    # bottom-left
                check_cell(x+1, y+1)   # bottom-right
            ]

            count = sum(cells)

            if count == 1 or count == 3:
                corners += 1
            elif count == 2 and ((cells[0] and cells[3]) or (cells[1] and cells[2])):
                corners += 2

    return corners


def main(input: str):
    grid: List[List[str]] = [list(row) for row in input.splitlines()]
    zones: ZonesType = defaultdict(set)
    id = 0
    points = set((x, y) for x in range(len(grid)) for y in range(len(grid[0])))

    # Fill zones
    while points:
        pos = next(iter(points))
        x, y = pos
        fill(grid, pos, (grid[x][y], id), zones)
        points = points - zones[(grid[x][y], id)]
        id += 1

    # Calculate original metrics
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

    # Calculate original total
    total = 0
    for zone in zones.values():
        num_sides = count_sides(grid, zone, zones)
        total += num_sides * len(zone)
    print(f"Total score: {total}")


if __name__ == '__main__':
    runSample = False
    CWD = Path(__file__).parent

    def readFile(filename):
        with open(filename, 'r') as f:
            return f.read()

    if (len(sys.argv) >= 2):
        runSample = sys.argv[1]

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
        start_time = timeit.default_timer()
        result = main(input)
        end_time = timeit.default_timer()
        print(f"Input execution time: {end_time - start_time:.4f} seconds")
