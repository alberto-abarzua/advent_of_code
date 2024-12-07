import sys
from pathlib import Path
import timeit

operators = {
    '*': lambda x, y: x * y,
    '+': lambda x, y: x + y,
    '||': lambda x, y: int(str(x) + str(y))
}


def check_equation(result, values, acc):
    if acc > result:
        return False

    if len(values) == 0:
        return result == acc
    for op in operators:
        new_acc = operators[op](acc, values[0])
        if check_equation(result, values[1:], new_acc):
            return True

    return False


def main(input: str):
    equations = [(result, values) for result, values in [line.split(": ") for line in input.splitlines()]]
    equations = [(int(result), [int(value) for value in values.split(" ")]) for result, values in equations]
    total = 0
    for result, values in equations:
        if check_equation(result, values[1:], values[0]):
            total += result
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
