import sys
from pathlib import Path
import timeit

directionMap = {
    ">": (0, 1),
    "<": (0, -1),
    "^": (-1, 0),
    "v": (1, 0)
}


def slide(grid, position, direction) -> bool:
    x, y = position
    dx, dy = directionMap[direction]
    nx, ny = (x + dx, y + dy)
    cur_cell = grid[x][y]
    assert nx in range(len(grid)) and ny in range(len(grid[0]))
    assert cur_cell == "O"

    next_cell = grid[nx][ny]
    if next_cell == "O":  # we need to slide all the Os
        res = slide(grid, (nx, ny), direction)
        if not res:
            return False
        grid[nx][ny] = "O"
        grid[x][y] = "."
        return True

    if next_cell == "#":
        return False

    if next_cell == ".":
        grid[nx][ny] = "O"
        grid[x][y] = "."
        return True

    raise Exception("Invalid cell", next_cell, cur_cell)


def move_to_pos(position, grid, move):
    x, y = position
    dx, dy = directionMap[move]
    nx, ny = (x + dx, y + dy)
    if nx not in range(len(grid)) or ny not in range(len(grid[0])):
        return None

    next_cell = grid[nx][ny]

    if next_cell == "#":
        return None

    if next_cell == "O":  # we need to slide all the Os
        res = slide(grid, (nx, ny), move)
        print("sliding", res)
        if not res:
            return None
        grid[nx][ny] = "@"
        grid[x][y] = "."
        return (nx, ny)

    if next_cell == ".":
        grid[nx][ny] = "@"
        grid[x][y] = "."
        return (nx, ny)


def main(input: str):
    grid, moves, *_ = input.split("\n\n")
    moves: str = "".join(moves.splitlines())
    grid = [list(row)for row in grid.splitlines()]
    # init pos is where @ is at
    init_pos = [(x, y) for x, row in enumerate(grid) for y, cell in enumerate(row) if cell == '@'][0]

    print("\n")
    print(moves)
    cur_pos = init_pos
    for m in moves:
        new_pos = move_to_pos(cur_pos, grid, m)
        if new_pos is not None:
            print(f"Moved {m} from {cur_pos} to {new_pos}")
            cur_pos = new_pos
        # print("\n")
        # for row in grid:
        #     print("".join(row))

    total = 0
    for x, row in enumerate(grid):
        for y, cell in enumerate(row):
            if cell == "O":
                total += x * 100 + y
    print(total)


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
