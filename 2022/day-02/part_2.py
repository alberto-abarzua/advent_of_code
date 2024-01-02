def main():
    input = open(0).read()
    m = {
        "A": 1,  # rock
        "B": 2,  # paper
        "C": 3,  # sicssors
        "X": 1,  # loose
        "Z": 2,  # win
        "Y": 3  # draw
    }

    vals = [[m[ch] for ch in x.split(" ")] for x in input.splitlines()]
    total = 0
    for (a, b) in vals:
        if a == 1:  # player a rock

            if b == 1:  
                total += 3
            if b == 2:  # paper
                total += 6 + 2
            if b == 3:  # draw
                total += 3 + 1

        if a == 2:  # player a paper

            if b == 1:
                total += 1
            if b == 2:
                total += 6 + 3
            if b == 3:
                total += 3 + 2

        if a == 3:  # player a sicssors

            if b == 1:
                total += 2
            if b == 2:
                total += 6 + 1
            if b == 3:
                total += 3 + 3

    return total


if __name__ == '__main__':
    print(main())
