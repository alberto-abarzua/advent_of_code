def move_close(head, tail):
    while True:
        _x = head[0] - tail[0]
        _y = head[1] - tail[1]

        if abs(_x) <= 1 and abs(_y) <= 1:
            break

        if _x == 0:
            tail = (tail[0], tail[1] + (_y // abs(_y)))
        elif _y == 0:
            tail = (tail[0] + (_x // abs(_x)), tail[1])
        else:
            tail = (tail[0] + (_x // abs(_x)), tail[1] + (_y // abs(_y)))


    return tail


def main():
    input = open(0).read()
    values = [[int(x) if x.isdigit() else x for x in line.split()] for line in input.splitlines()]
    visited = set()
    map = {
        "R": (1, 0),
        "L": (-1, 0),
        "U": (0, 1),
        "D": (0, -1)

    }

    head = (0, 0)
    tail = (0, 0)
    for (dir, val) in values:
        dir = map[dir]
        for _ in range(val):
            head = (head[0] + dir[0], head[1] + dir[1])
            tail = move_close(head, tail)
            visited.add(tail)


    return len(visited)


if __name__ == '__main__':
    print(main())
