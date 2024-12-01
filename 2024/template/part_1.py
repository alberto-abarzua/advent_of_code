import sys


def main(input: str):
    print("Running main function", input)
    return 0


def readFile(filename):
    with open(filename, 'r') as f:
        return f.read()


if __name__ == '__main__':
    runSample = False
    if (len(sys.argv) >= 2):
        runSample = sys.argv[1]

    print("Usage: python main.py <runSample>")
