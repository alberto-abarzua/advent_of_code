from enum import Enum
import sys
import timeit
from pathlib import Path
from collections import defaultdict


orientations = {
    "^": (-1, 0),
    "v": (1, 0),
    ">": (0, 1),
    "<": (0, -1)
}


def rotate90(orientation):
    if orientation not in orientations.keys():
        raise Exception("Invalid orientation")
    if orientation == "^":
        return ">"
    if orientation == ">":
        return "v"
    if orientation == "v":
        return "<"
    if orientation == "<":
        return "^"


class LoopDetected(Exception):
    pass


class MoveResult(Enum):
    LOOP = 1
    EXIT = 2


def move(position, grid, visited):
    x, y = position
    visited_key = (x, y, grid[x][y])

    if visited[visited_key]:
        return MoveResult.LOOP
    visited[visited_key] = True

    dx, dy = orientations[grid[x][y]]

    nx, ny = (x + dx, y + dy)

    if nx not in range(len(grid)) or ny not in range(len(grid[0])):
        grid[x][y] = 'X'
        return MoveResult.EXIT

    next_cell = grid[nx][ny]
    if next_cell == "#":
        if grid[x][y] not in orientations.keys():
            raise Exception("Invalid orientation")
        grid[x][y] = rotate90(grid[x][y])
        return move(position, grid, visited)

    grid[nx][ny] = grid[x][y]
    grid[x][y] = "X"

    return (nx, ny)


def initial_position(grid):
    for x, row in enumerate(grid):
        for y, cell in enumerate(row):
            if cell in orientations.keys():
                return (x, y)
    raise Exception("No initial position found")


def main(input: str):
    grid = [list(row)for row in input.splitlines()]
    init = initial_position(grid)
    if init is None:
        print("No initial position found")
        return
    cur_pos = init
    visited = defaultdict(lambda: False)
    grid_copy = [row.copy() for row in grid]
    while True:
        cur_pos = move(cur_pos, grid_copy, visited)
        if cur_pos is MoveResult.EXIT:
            break

    loops = 0
    grid_copy[init[0]][init[1]] = grid[init[0]][init[1]]
    for x, row in enumerate(grid_copy):
        for y, cell in enumerate(row):
            if cell == 'X':
                cur_pos = init
                visited = defaultdict(lambda: False)
                grid_copy = [row.copy() for row in grid]
                grid_copy[x][y] = '#'
                while True:
                    cur_pos = move(cur_pos, grid_copy, visited)
                    if cur_pos is MoveResult.EXIT:
                        break
                    if cur_pos is MoveResult.LOOP:
                        loops += 1
                        break
    print(loops)


def readFile(filename):
    with open(filename, 'r') as f:
        return f.read()


if __name__ == '__main__':
    runSample = False
    CWD = Path(__file__).parent

    # Parse command line arguments
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
