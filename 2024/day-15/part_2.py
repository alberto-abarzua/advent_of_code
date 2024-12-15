import sys
from pathlib import Path
import timeit

directionMap = {
    ">": (0, 1),
    "<": (0, -1),
    "^": (-1, 0),
    "v": (1, 0)
}

brackets = ["[", "]"]
opposite = {
    "[": "]",
    "]": "["
}

adjacent_list = {
    "^": {
        "]": (0, -1),
        "[": (0, 1)
    },
    "v": {
        "]": (0, -1),
        "[": (0, 1)
    }

}


def get_left_bracket(grid, postition, direction):
    try:
        x, y = postition
        a_dx, a_dy = adjacent_list[direction][grid[x][y]]
        ax, ay = (x + a_dx, y + a_dy)
        cur_cell = grid[x][y]
        adj_cur_cell = grid[ax][ay]
        if cur_cell == "[":
            return (x, y)
        if adj_cur_cell == "[":
            return (ax, ay)
        return None
    except:
        return None


def slide_vertical(grid, position, direction) -> bool:
    x, y = position
    dx, dy = directionMap[direction]
    nx, ny = (x + dx, y + dy)

    # Validate initial position contains a bracket
    if grid[x][y] != "[":
        raise Exception("Initial position must contain a bracket", grid[x][y], (x, y))

    # Check if next position is within grid bounds
    if nx not in range(len(grid)) or ny not in range(len(grid[0])):
        return False

    # Get the adjacent positions for the current and next cells
    a_dx, a_dy = adjacent_list[direction][grid[x][y]]
    ax, ay = (x + a_dx, y + a_dy)
    anx, any = (nx + a_dx, ny + a_dy)

    # Ensure adjacent next position is within bounds
    if anx not in range(len(grid)) or any not in range(len(grid[0])):
        return False

    next_cell = grid[nx][ny]
    next_adj_cell = grid[anx][any]

    # If either next cell is a wall, can't slide
    if next_cell == "#" or next_adj_cell == "#":
        return False

    # If both next cells are empty
    if next_cell == "." and next_adj_cell == ".":
        # Check if there's a matching bracket adjacent to current position
        if grid[ax][ay] != opposite[grid[x][y]]:
            return False

        # Move both brackets
        grid[nx][ny] = grid[x][y]
        grid[anx][any] = grid[ax][ay]
        grid[x][y] = "."
        grid[ax][ay] = "."
        return True

    # If either next cell is a bracket
    if next_cell in brackets or next_adj_cell in brackets:
        # Get the positions of the left brackets for both potential boxes
        box2 = get_left_bracket(grid, (nx, ny), direction)

        box1 = get_left_bracket(grid, (anx, any), direction)

        # Try to slide the next box first
        print(f"Sliding {box2} {direction} {grid[nx][ny]}")
        print(f"Sliding {box1} {direction} {grid[anx][any]}")
        boxes = set([box1, box2])
        grid_copy = [row[:] for row in grid[:]]
        for box in boxes:
            if box is None:
                continue
            res = slide(grid_copy, box, direction)
            if not res:
                return False
        grid = [row[:] for row in grid_copy[:]]

        # Then move the current box
        grid[nx][ny] = grid[x][y]
        grid[anx][any] = grid[ax][ay]
        grid[x][y] = "."
        grid[ax][ay] = "."
        return True

    # If we reach here, there's an invalid cell combination
    raise Exception(f"Unexpected cell combination: {next_cell}, {next_adj_cell}, {(nx,ny)}")


def slide_horizontal(grid, position, direction) -> bool:
    x, y = position
    dx, dy = directionMap[direction]
    nx, ny = (x + dx, y + dy)
    if grid[x][y] not in brackets:
        print(f"NOT IN BRACKETS {grid[x][y]}")
        raise Exception()
    assert nx in range(len(grid)) and ny in range(len(grid[0]))

    next_cell = grid[nx][ny]

    if next_cell in brackets:
        res = slide(grid, (nx, ny), direction)
        if not res:
            return False
        grid[nx][ny] = grid[x][y]
        grid[x][y] = "."
        return True

    if next_cell == "#":
        return False

    if next_cell == ".":
        grid[nx][ny] = grid[x][y]
        grid[x][y] = "."
        return True

    raise Exception()


def slide(grid, position, direction) -> bool:
    if direction in [">", "<"]:
        return slide_horizontal(grid, position, direction)
    if direction in ["^", "v"]:
        if grid[position[0]][position[1]] in brackets:
            left_bracket = get_left_bracket(grid, position, direction)
            if left_bracket is None:
                return False
            return slide_vertical(grid, left_bracket, direction)
        return slide_vertical(grid, position, direction)
    raise Exception()


def move_to_pos(position, grid, move):
    x, y = position
    dx, dy = directionMap[move]
    nx, ny = (x + dx, y + dy)
    if nx not in range(len(grid)) or ny not in range(len(grid[0])):
        return None

    next_cell = grid[nx][ny]

    if next_cell == "#":
        return None

    print(f"Next cell {next_cell} {move} {x,y}")
    print(next_cell in brackets)
    if next_cell in brackets:  # we need to slide all the Os
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
    new_grid = []
    for row in grid:
        new_row = []
        for cell in row:
            if cell == "O":
                new_row.extend(["[", "]"])
            if cell == "#":
                new_row.extend(["#", "#"])
            if cell == ".":
                new_row.extend([".", "."])
            if cell == "@":
                new_row.extend(["@", "."])
        new_grid.append(new_row)
    # [print("".join(row)) for row in new_grid]

    # init pos is where @ is at

    grid = new_grid
    init_pos = [(x, y) for x, row in enumerate(grid) for y, cell in enumerate(row) if cell == '@'][0]

    print("\n")
    print(moves)
    cur_pos = init_pos
    for m in moves:
        new_pos = move_to_pos(cur_pos, grid, m)
        if new_pos is not None:
            print(f"Moved {m} from {cur_pos} to {new_pos}")
            cur_pos = new_pos
        print("\n")
        # for row in grid:
        #     print("".join(row))

    total = 0
    for x, row in enumerate(grid):
        for y, cell in enumerate(row):
            if cell == "[":
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
