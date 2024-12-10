import sys
from pathlib import Path
import timeit
import heapq
from collections import defaultdict
from typing import List, Tuple, Dict


directions = {
    'up': (-1, 0),
    'down': (1, 0),
    'left': (0, -1),
    'right': (0, 1)
}


def number_of_paths_between_points(graph: List[List[int]], start: Tuple[int, int], end: Tuple[int, int]) -> int:
    directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]  # up, down, left, right
    rows, cols = len(graph), len(graph[0])
    total_paths = 0

    def dfs(current: Tuple[int, int]):
        nonlocal total_paths
        if current == end:
            total_paths += 1
            return

        x, y = current
        for dx, dy in directions:
            neighbor = (x + dx, y + dy)
            nx, ny = neighbor

            if nx in range(rows) and ny in range(cols):
                # Ensure neighbor's weight is exactly 1 greater than current
                if graph[nx][ny] - graph[x][y] == 1:
                    dfs(neighbor)

    dfs(start)
    return total_paths


def dijkstra(graph: List[List[int]], start: Tuple[int, int]):
    distances = defaultdict(lambda: float('inf'))
    previous_vertices: Dict[Tuple[int, int], Tuple[int, int] | None] = defaultdict(lambda: None)
    final_points = set()

    distances[start] = 0
    priority_queue = [(0, start)]
    heapq.heapify(priority_queue)

    while len(priority_queue) > 0:
        current_distance, current_vertex = heapq.heappop(priority_queue)
        cx, cy = current_vertex

        if current_distance > distances[current_vertex]:
            continue

        # neighbors are only up down left right where the int diff is 1 (positive)
        for (dx, dy) in directions.values():
            neighbor = (cx + dx, cy + dy)
            nx, ny = neighbor
            current_weight = graph[cx][cy]

            if nx not in range(len(graph)) or ny not in range(len(graph[0])):
                continue

            neighbor_weight = graph[nx][ny]
            if neighbor_weight-current_weight != 1:
                continue

            distance = current_distance + 1

            if distance < distances[neighbor]:
                distances[neighbor] = distance
                previous_vertices[neighbor] = current_vertex
                heapq.heappush(priority_queue, (distance, neighbor))
                if neighbor_weight == 9:
                    final_points.add(neighbor)
    return distances, previous_vertices, final_points 


def main(input: str):
    graph = [[int(x) for x in row] for row in input.splitlines()]
    start = (0, 2)
    [print(row) for row in graph]
    total = 0
    for x, row in enumerate(graph):
        for y, cell in enumerate(row):
            start = (x, y)
            if cell != 0:
                continue
            distances, previous_vertices, final_points = dijkstra(graph, start)

            for final_point in final_points:
                total += number_of_paths_between_points(graph, start, final_point)

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
