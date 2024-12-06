import sys
from pathlib import Path


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


def move(position, grid):
    x, y = position

    dx, dy = orientations[grid[x][y]]

    nx, ny = (x + dx, y + dy)

    if nx not in range(len(grid)) or ny not in range(len(grid[0])):
        return None

    next_cell = grid[nx][ny]
    if next_cell == "#":
        if grid[x][y] not in orientations.keys():
            raise Exception("Invalid orientation")
        grid[x][y] = rotate90(grid[x][y])
        return move(position, grid)

    grid[nx][ny] = grid[x][y]
    grid[x][y] = "X"

    return (nx, ny)


def initial_position(grid):
    for x, row in enumerate(grid):
        for y, cell in enumerate(row):
            if cell in orientations.keys():
                return (x, y)

    return None


def main(input: str):
    grid = [list(row)for row in input.splitlines()]
    init = initial_position(grid)
    if init is None:
        print("No initial position found")
        return
    cur_pos = init
    while True:
        cur_pos = move(cur_pos, grid)
        if cur_pos is None:
            break
    total = sum([row.count("X") for row in grid])
    print(total+1)


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
        main(sample)
    else:
        print("Running input")
        main(input)
