from collections import defaultdict
import sys
from pathlib import Path
import timeit
from dataclasses import dataclass
from typing import List, Tuple
import re


@dataclass
class Robot:
    initial: Tuple[int, int]
    velocity: Tuple[int, int]
    position: Tuple[int, int]

    def __str__(self):
        return f"Robot I: {self.initial}  V: {self.velocity} P: {self.position})"

    def __repr__(self):
        return self.__str__()

    def calc_pos(self, time: int) -> Tuple[int, int]:
        x, y = self.initial
        dx, dy = self.velocity
        return (x + dx*time, y + dy*time)

    def get_quadrant(self, t_wide: int, t_high: int) -> int:
        x, y = self.position

        if x == t_wide//2 or y == t_high//2:
            return 0

        if x < t_wide/2 and y < t_high/2:
            return 1
        if x >= t_wide/2 and y < t_high/2:
            return 2
        if x < t_wide/2 and y >= t_high/2:
            return 3
        return 4


def max_continuous_subarray(arr: List[str], char):
    max_count = 0
    count = 0
    for i in arr:
        if i == char:
            count += 1
        else:
            count = 0
        max_count = max(max_count, count)
    return max_count


def main(input: str):
    r_data = [row.split() for row in input.splitlines()]
    robots = []
    for init_data, speed_data in r_data:
        initial = tuple(map(int, re.findall(r'[+-]?\d+', init_data)))
        velocity = tuple(map(int, re.findall(r'[+-]?\d+', speed_data)))
        assert len(initial) == 2
        assert len(velocity) == 2
        robots.append(Robot(initial, velocity, initial))

    # [print(robot) for robot in robots]
    # t_wide = 11
    # t_high = 7
    t_wide = 101
    t_high = 103
    t_time = 10000

    for i in range(t_time):
        grid = [['_' for _ in range(t_wide)] for _ in range(t_high)]
        for robot in robots:
            x, y = robot.calc_pos(i)
            x, y = x % t_wide, y % t_high
            # print(f"Robot {robot} at {x}, {y} at time {i}")
            robot.position = (x, y)
            grid[y][x] = "X"

        max_conts = [max_continuous_subarray(row, "X") for row in grid]
        max_total = max(max_conts)
        if max_total >= 10:
            print(f"Time: {i}")
            print(f"------------ time {i} ")
            [print(''.join(row)) for row in grid]
            print("------------")

    # [print(robot) for robot in robots]
    quadrants = defaultdict(lambda: 0)
    for robot in robots:
        q = robot.get_quadrant(t_wide, t_high)
        quadrants[q] += 1
    # remove 0
    for key, value in quadrants.items():
        print(f"Quadrant {key}: {value}")
    res = quadrants[1]*quadrants[2]*quadrants[3]*quadrants[4]
    print(res)


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
